pub mod modifiers;
pub mod types;

use std::fmt::Display;

pub use modifiers::StationModifiers;

use crate::{util::*, Pallet, *};
use types::{StationType, STATION_TYPES};

/// Instance of a station
#[derive(Debug, Clone)]
pub struct Station {
    /// Location of the station in source code
    pub loc: SourceSpan,
    /// Station type information
    pub s_type: &'static StationType,
    /// Data the station may need
    pub data: StationData,
    /// Modifiers
    pub modifiers: StationModifiers,
    /// In bay list, in the form (priority, pallet)
    pub in_bays: Vec<(u32, Pallet)>,
    /// Map of each output bay connection in the form (station_index, in_bay_priority)
    pub out_bays: Vec<(usize, u32)>,
}
impl Station {
    pub fn new(loc: SourceSpan, s_type: &'static StationType) -> Self {
        Station {
            loc,
            s_type,
            data: StationData::None,
            modifiers: StationModifiers::default(),
            in_bays: Vec::new(),
            out_bays: Vec::new(),
        }
    }

    pub fn with_data(self, data: StationData) -> Self {
        Station { data, ..self }
    }

    pub fn with_modifiers(self, modifiers: StationModifiers) -> Self {
        Station { modifiers, ..self }
    }

    pub fn from_str(identifier: &str, loc: SourceSpan) -> Result<Self, Error> {
        for station_type in STATION_TYPES.iter() {
            if station_type.has_id(identifier) {
                return Ok(Self {
                    loc,
                    s_type: *station_type,
                    modifiers: StationModifiers::default(),
                    in_bays: Vec::new(),
                    out_bays: Vec::new(),
                    data: StationData::None,
                });
            }
        }
        return Err(Error::new(
            IdentifierError,
            loc,
            format!("Failed to find station type with identifier \"{identifier}\"").as_str(),
        ));
    }

    /// Checks whether this station is ready to be triggered (if the length of inputs is >= the number of inputs needed)
    pub fn ready(&self, program: &FSProgram) -> bool {
        let len = self.in_bays.len();
        if self.s_type == &types::FUNC_INVOKE {
            // if its a function invocation, check the number of args the function needs
            let function_id = if let StationData::FunctionID(id) = self.data {
                id
            } else {
                panic!();
            };
            return len >= program.function_templates[function_id].n_args;
        } else if self.s_type == &types::MAIN || self.s_type == &types::FUNC_INPUT {
            // these stations can't trigger
            return false;
        }
        return len >= self.s_type.inputs;
    }

    /// collects all the input pallets into a vector and clears the input bays
    pub fn get_input_pallets(&mut self) -> Vec<Pallet> {
        self.in_bays.sort_by_key(|p| p.0);

        let mut pallets: Vec<Pallet> = Vec::with_capacity(self.in_bays.len());
        for (_, pallet) in self.in_bays.iter() {
            pallets.push(pallet.clone());
        }

        self.in_bays.clear();
        return pallets;
    }

    /// send a pallet to this stations bay
    pub fn send_pallet(&mut self, pallet: Pallet, priority: u32) {
        // checking for duplicates
        for item in self.in_bays.iter_mut() {
            if item.0 == priority {
                item.1 = pallet;
                return;
            }
        }
        self.in_bays.push((priority, pallet));
    }
}
impl Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} @ {}  {}",
            self.s_type,
            self.loc,
            match &self.data {
                StationData::AssignValue(val) => format!("({val})"),
                StationData::FunctionID(id) => format!("(function {id})"),
                StationData::FunctionIDAndIndex(id, arg_i) =>
                    format!("(function {id}, arg {arg_i})"),
                StationData::None => String::new(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum StationData {
    AssignValue(Pallet),
    FunctionID(usize),
    FunctionIDAndIndex(usize, usize),
    None,
}
