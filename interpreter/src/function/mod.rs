use crate::{station::*, *};

/// Template of a function, used to instantiate a function when it is invoked
#[derive(Clone)]
pub struct FunctionTemplate {
    pub stations: Vec<Station>,
    pub n_args: usize,
    pub name: String,
}
impl FunctionTemplate {
    pub fn new(name: String) -> Self {
        FunctionTemplate {
            stations: Vec::new(),
            n_args: 0,
            name,
        }
    }
}

/// Instance of a function
pub struct Function<'a> {
    /// Template that this function is an instance of
    pub template: &'a FunctionTemplate,
    /// Parent program
    program: &'a FSProgram,
    /// Stations local to this function
    stations: Vec<Station>,
    /// Vector of all pallets to move in the next step, contains tuple with the pallet and the destination index and bay priority
    moving_pallets: Vec<(Pallet, (usize, u32))>,
    /// All functions spawned from this one
    children: Vec<Function<'a>>,
    /// Output of this function
    output: Option<Pallet>,
    /// Index of station that invoked this function
    parent_station: usize,
    /// Recursion depth
    depth: u32,
}
impl<'a> Function<'a> {
    /// Performs one time step (returns true if an exit station was triggered)
    pub fn step(&mut self) -> Result<bool, Error> {
        debug!(
            4,
            "Stepping function {} (depth={})", self.template.name, self.depth
        );
        // moving the pallets
        for (pallet, (dest_i, priority)) in self.moving_pallets.iter() {
            debug!(5, " - {pallet} moved to station {dest_i} ({priority})");
            self.stations[*dest_i]
                .in_bays
                .push((pallet.clone(), *priority));
        }
        self.moving_pallets.clear();

        // executing station procedures
        for i in 0..self.stations.len() {
            let station = &mut self.stations[i];

            if !station.ready(&self.program) {
                continue; // not enough inputs to trigger procedure
            }
            let input: Vec<Pallet> = station.get_input_pallets();

            debug!(
                5,
                " - Procedure triggered on station {i} {} ({} pallets)",
                station.s_type,
                input.len()
            );

            if station.s_type == &station::types::FUNC_INVOKE {
                // special case: function invocation
                let function_template = if let StationData::FunctionID(id) = station.data {
                    if self.depth >= MAX_RECURSION_DEPTH {
                        return Err(Error::new(
                            RuntimeError,
                            station.loc,
                            format!(
                                "Max recursion depth hit during invocation of function '{}'",
                                self.program.function_templates[id].name
                            ),
                        ));
                    }
                    &self.program.function_templates[id]
                } else {
                    panic!();
                };

                debug!(4, "Invoking '{}'", function_template.name);
                self.invoke(function_template, input, i);
                continue;
            } else if station.s_type == &station::types::FUNC_OUTPUT {
                // special case: function output
                self.output = Some(input[0].clone());
                continue;
            } else if station.s_type == &station::types::ASSIGN {
                // special case: assign station
                if let StationData::AssignValue(p) = &station.data {
                    debug!(5, "    - Produced: {}", p);
                    for out_bay in station.out_bays.iter() {
                        self.moving_pallets.push((p.clone(), *out_bay));
                    }
                } else {
                    return Err(Error::new(
                        RuntimeError,
                        station.loc,
                        format!("Can't fetch assign value"),
                    ));
                };
                continue;
            } else if station.s_type == &station::types::EXIT {
                // special case: exit station
                debug!(2, "Exit triggered by station");
                return Ok(true);
            }

            // running procedures
            match (station.s_type.procedure)(input) {
                Ok(Some(p)) => {
                    debug_assert!(station.s_type.output == true, "Unexpected pallet returned");
                    debug!(5, "    - produced: {}", p);
                    for out_bay in station.out_bays.iter() {
                        self.moving_pallets.push((p.clone(), *out_bay));
                    }
                }
                Ok(None) => {
                    debug!(5, "    - produced: None",);
                }
                Err(msg) => {
                    return Err(Error::new(RuntimeError, station.loc, msg));
                }
            }
        }

        debug!(
            4,
            "function '{}' (depth={}), stepping children", self.template.name, self.depth
        );
        // stepping children
        for child in self.children.iter_mut() {
            if child.step()? {
                return Ok(true);
            }
        }

        // checking if children are done executing
        self.children.retain(|child| {
            if child.is_done() {
                debug!(
                    4,
                    "child function '{}' (depth={}) finished", child.template.name, self.depth
                );
                if let Some(output) = child.output.clone() {
                    for dest in self.stations[child.parent_station].out_bays.iter() {
                        self.moving_pallets.push((output.clone(), *dest));
                    }
                }
                return false;
            }
            return true;
        });

        return Ok(false);
    }

    /// Instantiates a function template as a child
    pub fn invoke(
        &mut self,
        template: &'a FunctionTemplate,
        input: Vec<Pallet>,
        parent_station: usize,
    ) {
        debug_assert_eq!(
            input.len(),
            template.n_args,
            "Not provided enough inputs to func_invoke"
        );
        let mut f = Function {
            template,
            program: self.program,
            stations: template.stations.clone(),
            moving_pallets: Vec::new(),
            children: Vec::new(),
            output: None,
            depth: self.depth + 1,
            parent_station,
        };
        for s in f.stations.iter() {
            if let StationData::FunctionIDAndIndex(_, arg_index) = s.data {
                // station is function input
                for dest in s.out_bays.iter() {
                    f.moving_pallets.push((input[arg_index].clone(), *dest));
                }
            }
        }
        self.children.push(f);
    }

    /// Returns whether or not this function is done executing
    pub fn is_done(&self) -> bool {
        return (self.moving_pallets.is_empty() && self.children.is_empty())
            || self.output.is_some();
    }

    /// Instantiates the main function, used for program initialization
    pub fn instantiate_main(program: &'a FSProgram) -> Self {
        let template = &program.main;
        let mut f = Function {
            template,
            program,
            moving_pallets: Vec::new(),
            stations: template.stations.clone(),
            children: Vec::new(),
            output: None,
            depth: 0,
            parent_station: 0,
        };

        // spawning start pallets
        for s in f.stations.iter() {
            if s.s_type == &station::types::MAIN {
                for dest in s.out_bays.iter() {
                    f.moving_pallets.push((Pallet::Empty, *dest));
                }
            }
        }

        return f;
    }
}
