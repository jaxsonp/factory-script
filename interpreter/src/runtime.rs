use std::time::Instant;

use crate::{
    debug,
    pallet::Pallet,
    station::{self, types::StationType, Station, StationData},
    *,
};

/// Spawns pallets from the start station and starts the execution loop, returns
/// the number of steps in the program
pub fn execute(stations: &mut Vec<Station>, step_count: &mut u64) -> Result<(), Error> {
    // Vector of all pallets to move in the next step, tuple with the pallet and
    // the destination index and bay number
    let mut moving_pallets: Vec<(Pallet, (usize, usize))> = Vec::new();

    *step_count = 0;

    'execution_loop: while !moving_pallets.is_empty() {
        // recording start time of iteration
        let step_start_t = Instant::now();

        // moving the pallets
        for (pallet, dest) in moving_pallets.iter() {
            debug!(3, " - pallet moved to #{}:{} ({})", dest.0, dest.1, pallet);
            stations[dest.0].in_bays[dest.1] = Some(pallet.clone());
        }
        moving_pallets.clear();

        // executing station procedures
        for i in 0..stations.len() {
            // counting occupied bays
            let mut input: Vec<&Pallet> = Vec::new();
            for bay in stations[i].in_bays.iter() {
                if let Some(p) = bay {
                    input.push(p)
                }
            }

            match stations[i].s_type {
                StationType::Builtin(s_type) => {
                    // station is a builtin station
                    if input.len() < s_type.inputs {
                        continue; // not enough inputs to trigger procedure
                    }
                    // running procedures
                    debug!(3, " - Procedure triggered on #{i} ({})", stations[i].s_type);

                    // special case: assign station
                    if s_type == &station::types::builtin::ASSIGN {
                        if let StationData::AssignValue(p) = &stations[i].data {
                            debug!(4, "    - Produced: {}", p);
                            for out_bay in stations[i].out_bays.iter() {
                                moving_pallets.push((p.clone(), *out_bay));
                            }
                        } else {
                            return Err(Error::new(
                                RuntimeError,
                                stations[i].loc,
                                format!("Can't find assign value for #{i}"),
                            ));
                        };
                        stations[i].clear_in_bays();
                        continue;
                    } else if s_type == &station::types::builtin::EXIT {
                        debug!(2, "Exit triggered by station");
                        break 'execution_loop;
                    }

                    match (s_type.procedure)(input) {
                        Ok(Some(p)) => {
                            debug_assert!(s_type.output == true, "Unexpected pallet returned");
                            debug!(4, "    - produced: {}", p);
                            for out_bay in stations[i].out_bays.iter() {
                                moving_pallets.push((p.clone(), *out_bay));
                            }
                        }
                        Ok(None) => {
                            debug!(4, "    - produced: None",);
                        }
                        Err(msg) => {
                            return Err(Error::new(RuntimeError, stations[i].loc, msg));
                        }
                    }
                }
                StationType::FuncInvoke(id) => {}
                StationType::FuncInput((id, arg_index)) => {}
                StationType::FuncOutput(id) => {}
            }
            stations[i].clear_in_bays();
        }
        debug!(
            3,
            "Step {step_count} completed ({:.3} ms)",
            step_start_t.elapsed().as_secs_f64() * 1000.0
        );

        *step_count += 1;
    }
    debug!(2, "No remaining moving pallets");

    return Ok(());
}
