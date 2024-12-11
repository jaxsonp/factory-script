pub mod conveyor_belt_parser;
pub mod station_parser;

use crate::{station::Station, util::*, *};

pub const BELT_CHARS: &str = "─│┌┐└┘═║╔╗╚╝";
pub const SINGLE_BELT_CHARS: &str = "─│┌┐└┘";
pub const DOUBLE_BELT_CHARS: &str = "═║╔╗╚╝";
pub const NORTH_BELT_CHARS: &str = "│└┘║╚╝";
pub const EAST_BELT_CHARS: &str = "─┌└═╔╚";
pub const SOUTH_BELT_CHARS: &str = "│┌┐║╔╗";
pub const WEST_BELT_CHARS: &str = "─┐┘═╗╝";

/// Preprocesses a source string, validating the syntax and grammar
///
/// Returns a tuple containing a vector of stations and the start station index
pub fn process<'a>(src: &str) -> Result<Vec<Station>, Error> {
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
    let mut stations = station_parser::parse_stations(&char_map)?;
    debug!(3, "Found {} stations", stations.len());

    // parsing conveyor belt connections
    conveyor_belt_parser::parse_conveyor_belts(&char_map, &mut stations)?;

    debug!(2, "Finished preprocessing");
    Ok(stations)
}
