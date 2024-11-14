use std::collections::HashMap;

pub mod conveyor_belt_parser;
pub mod station_parser;

use crate::*;
use fs_core::Pallet;

#[cfg(test)]
mod tests;

/// Preprocesses a source string, validating the syntax and grammar
///
/// Returns a tuple containing a vector of stations and the assignment table, which
/// store the index of every assign station and its corresponding assign value
pub fn process<'a>(src: &str) -> Result<(Vec<Station>, usize, HashMap<usize, Pallet>), Error> {
    // generating 2d vector layout of source code
    let mut char_map: Vec<Vec<char>> = Vec::new();
    let mut n_chars = 0;
    for line in src.split('\n') {
        let row = line.chars().collect::<Vec<char>>();
        n_chars += row.len();
        char_map.push(row);
    }
    if n_chars == 0 {
        return Err(Error::new(SyntaxError, SourcePos::zero(), "Empty file"));
    }

    // station discovery
    debug!(3, "Discovering stations");
    let (mut stations, assign_table) = station_parser::parse_stations(&char_map)?;
    debug!(3, "Found {} stations", stations.len());

    // getting start station's index
    let mut start_i: usize = 0;
    let mut found_start = false;
    for i in 0..stations.len() {
        if stations[i].logic.id == "start" {
            if found_start {
                return Err(Error::new(
                    SyntaxError,
                    stations[i].loc,
                    "Found multiple start stations",
                ));
            }
            start_i = i;
            found_start = true;
            break;
        }
    }
    if !found_start {
        return Err(Error::new(
            SyntaxError,
            SourcePos::zero(),
            "Unable to locate start station",
        ));
    }

    // parsing conveyor belt connections
    conveyor_belt_parser::parse_conveyor_belts(&char_map, &mut stations)?;

    debug!(2, "Finished preprocessing");
    Ok((stations, start_i, assign_table))
}
