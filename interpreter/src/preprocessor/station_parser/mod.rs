mod literal_parser;

use crate::*;
use core::*;
use literal_parser::parse_assign_literal;

#[cfg(test)]
mod tests;

enum State {
    Default,
    Station,
    StationModifiers(StationModifiers),
    AssignStation,
}

/// function to increment the position in the character map and get the next character
/// there, if there is one
fn get_next_char(pos: &mut SourcePos, char_map: &Vec<Vec<char>>) -> Option<char> {
    pos.col += 1;
    while pos.col >= char_map[pos.line].len() {
        pos.col = 0;
        pos.line += 1;
        if pos.line >= char_map.len() {
            return None;
        }
    }
    let c = char_map[pos.line][pos.col];
    return Some(c);
}

/// Identifies stations using a finite state machine. Returns a vector of stations
/// discovered, and the assign table
pub fn parse_stations(char_map: &Vec<Vec<char>>) -> Result<Vec<Station>, Error> {
    let mut stations: Vec<Station> = Vec::new();

    let mut pos = SourcePos::zero();
    // getting first character
    let mut c: char = loop {
        if pos.line >= char_map.len() {
            return Err(Error::new(
                SyntaxError,
                SourcePos::zero(),
                "Empty factory file",
            ));
        }
        if char_map[pos.line].len() > 0 {
            break char_map[pos.line][0];
        }
        pos.line += 1;
    };

    // finite state machine's current state
    let mut state: State = State::Default;

    // persistent variables
    let mut cur_token = String::new();
    let mut cur_station_pos = SourcePos::zero();

    loop {
        // incrementing the state machine
        match state {
            State::Default => match c {
                // start of station
                '[' => {
                    state = State::Station;
                    cur_token = String::new();
                    cur_station_pos = pos.clone();
                    debug!(4, "   - station start @ {}", pos);
                }
                // start of assign station
                '{' => {
                    state = State::AssignStation;
                    cur_token = String::new();
                    cur_station_pos = pos.clone();
                    debug!(4, "   - assign station start @ {}", pos);
                }
                // ehhh???
                ']' | '}' => {
                    return Err(Error::new(SyntaxError, pos, "Unexpected closing bracket"));
                }
                // non station stuff (conveyor belts, floating comments)
                _ => {}
            },
            State::Station => {
                if c == ']' {
                    // new station w no modifiers
                    debug!(4, "   - station end @ {}", pos);
                    let new_station = Station::new(
                        cur_token.as_str(),
                        SourceSpan::new(cur_station_pos, cur_token.len() + 2),
                    )?;
                    debug!(
                        3,
                        " - #{} {} @ {}",
                        stations.len(),
                        new_station.logic.id,
                        new_station.loc
                    );
                    stations.push(new_station);
                    state = State::Default;
                } else if c == ':' {
                    // start of modifiers
                    state = State::StationModifiers(StationModifiers::default());
                    debug!(4, "   - station modifiers @ {}", pos);
                } else if c.is_ascii_graphic() && !c.is_ascii_whitespace() {
                    // station identifier
                    cur_token.push(c);
                } else {
                    // invalid character
                    return Err(Error::new(
                        SyntaxError,
                        pos,
                        "Invalid character, station identifiers only contain non-whitespace, printable ASCII characters",
                    ));
                }
            }
            State::StationModifiers(ref mods) => match c {
                'N' => state = State::StationModifiers(mods.with_priority(Direction::NORTH)),
                'E' => state = State::StationModifiers(mods.with_priority(Direction::EAST)),
                'S' => state = State::StationModifiers(mods.with_priority(Direction::SOUTH)),
                'W' => state = State::StationModifiers(mods.with_priority(Direction::WEST)),
                '*' => state = State::StationModifiers(mods.reverse()),
                ']' => {
                    debug!(4, "   - station end @ {}", pos);
                    let mut new_station = Station::new(
                        cur_token.as_str(),
                        SourceSpan::new(cur_station_pos, pos.col - cur_station_pos.col),
                    )?;
                    new_station.modifiers = *mods;
                    debug!(
                        3,
                        " - #{} {} @ {}",
                        stations.len(),
                        new_station.logic.id,
                        new_station.loc
                    );
                    stations.push(new_station);
                    state = State::Default;
                }
                _ => {
                    // invalid character
                    return Err(Error::new(
                        SyntaxError,
                        pos,
                        "Invalid modifier character, acceptable modifiers are 'N', 'S', 'E', 'W' and '~'",
                    ));
                }
            },
            State::AssignStation => match c {
                '}' => {
                    debug!(4, "   - station end @ {}", pos);
                    // creating new station
                    let loc = SourceSpan::new(cur_station_pos, pos.col - cur_station_pos.col + 1);
                    let assign_val = parse_assign_literal(&cur_token, loc)?;
                    let new_station = Station::new_assign(assign_val.clone(), loc);
                    // parsing literal type
                    debug!(
                        3,
                        " - #{} {} @ {} ({})",
                        stations.len(),
                        new_station.logic.id,
                        new_station.loc,
                        assign_val
                    );
                    stations.push(new_station);
                    state = State::Default;
                }
                '\\' => {
                    //escaped chars
                    cur_token.push(match get_next_char(&mut pos, char_map) {
                        Some('n') => '\n',
                        Some('r') => '\r',
                        Some('t') => '\t',
                        Some('\\') => '\\',
                        Some('\'') => '\'',
                        Some('"') => '"',
                        Some('}') => '}',
                        Some(c) => c,
                        None => {
                            return Err(Error::new(SyntaxError, pos, "Unexpected EOF"));
                        }
                    });
                }
                c => {
                    cur_token.push(c);
                }
            },
        }

        // getting next char
        c = match get_next_char(&mut pos, char_map) {
            Some(c) => c,
            None => {
                debug!(4, "   - EOF reached");
                break;
            }
        };
    }
    match state {
        State::Default => {
            return Ok(stations);
        }
        _ => return Err(Error::new(SyntaxError, cur_station_pos, "Unexpected EOF")),
    }
}
