pub mod connection_parser;
pub mod station_parser;

use std::collections::HashSet;

use station::StationData;

use crate::{station::Station, util::*, *};

pub const BELT_CHARS: &str = "─│┌┐└┘═║╔╗╚╝";
pub const SINGLE_BELT_CHARS: &str = "─│┌┐└┘";
pub const DOUBLE_BELT_CHARS: &str = "═║╔╗╚╝";
pub const NORTH_BELT_CHARS: &str = "│└┘║╚╝";
pub const EAST_BELT_CHARS: &str = "─┌└═╔╚";
pub const SOUTH_BELT_CHARS: &str = "│┌┐║╔╗";
pub const WEST_BELT_CHARS: &str = "─┐┘═╗╝";

/// Preprocesses a source string, validating/parsing the syntax and grammar
pub fn process<'a>(src: &str) -> Result<FSProgram, Error> {
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

    // finding all stations
    debug!(3, "Parsing stations");
    let (stations, mut functions) = station_parser::parse_stations(&char_map)?;
    debug!(2, "Found {} stations", stations.len());

    // parsing connections between stations
    connection_parser::parse(&char_map, stations, &mut functions)?;

    // validating functions
    for f in functions.iter_mut() {
        let mut args_seen: HashSet<usize> = HashSet::new();
        for s in f.stations.iter() {
            if let StationData::FunctionIDAndIndex(_, arg_i) = s.data {
                // station is an function input station
                if args_seen.contains(&arg_i) {
                    return Err(Error::new(
                        SyntaxError,
                        s.loc,
                        "Duplicate function arguments",
                    ));
                }

                // number of args is the highest seen argument number
                args_seen.insert(arg_i);
                if arg_i + 1 >= f.n_args {
                    f.n_args = arg_i + 1;
                }
            }
        }
    }

    debug!(2, "Finished preprocessing");

    Ok(FSProgram {
        main: functions[0].clone(),
        function_templates: functions,
        benchmark: false,
    })
}
