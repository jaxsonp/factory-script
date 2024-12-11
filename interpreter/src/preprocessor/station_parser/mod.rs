mod literal_parser;

use std::collections::HashMap;

use crate::{station::*, util::*, *};
use literal_parser::parse_assign_literal;

enum State {
    Default,
    Station,
    StationModifiers(StationModifiers),
    FunctionName,
    FunctionSuffix(usize),
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

    // map to ID functions
    let mut function_ids: HashMap<String, usize> = HashMap::new();
    let mut n_functions = 0;

    let mut pos = SourcePos::zero();
    // getting first character
    let mut c: char = loop {
        if pos.line >= char_map.len() {
            return Err(Error::new(
                SyntaxError,
                SourcePos::zero(),
                "Empty factory program",
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
                }
                // start of assign station
                '{' => {
                    state = State::AssignStation;
                    cur_token = String::new();
                    cur_station_pos = pos.clone();
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
                    let new_station = Station::from_str(
                        cur_token.as_str(),
                        SourceSpan::new(cur_station_pos, cur_token.len() + 2),
                    )?;
                    debug!(3, " - {} @ {}", new_station.s_type, new_station.loc);
                    stations.push(new_station);
                    state = State::Default;
                } else if c == '$' {
                    // function related station
                    if !cur_token.is_empty() {
                        // dollar sign in middle of station syntax
                        return Err(Error::new(
                            SyntaxError,
                            pos,
                            "Invalid '$' character, must be at beginning of station declaration",
                        ));
                    }
                    state = State::FunctionName;
                } else if c == ':' {
                    // start of modifiers
                    state = State::StationModifiers(StationModifiers::default());
                } else if c.is_ascii_graphic() && !c.is_ascii_whitespace() {
                    // station identifier
                    cur_token.push(c);
                } else {
                    // invalid character
                    return Err(Error::new(
                        SyntaxError,
                        pos,
                        "Invalid character, station identifiers can only contain non-whitespace, printable ASCII characters",
                    ));
                }
            }
            State::FunctionName => {
                if c == '.' || c == ']' {
                    // done reading function name
                    let function_id: usize;
                    match function_ids.get(&cur_token) {
                        Some(id) => {
                            function_id = *id;
                        }
                        None => {
                            function_ids.insert(cur_token.clone(), n_functions);
                            function_id = n_functions;
                            n_functions += 1;
                        }
                    }
                    if c == '.' {
                        // function input or output
                        state = State::FunctionSuffix(function_id);
                    } else {
                        // function invocation
                        let new_station = Station {
                            loc: SourceSpan::new(cur_station_pos, pos.col - cur_station_pos.col),
                            s_type: &station::types::FUNC_INVOKE,
                            modifiers: StationModifiers::default(),
                            in_bays: Vec::new(),
                            out_bays: Vec::new(),
                            data: StationData::FunctionID(function_id),
                        };
                        debug!(
                            3,
                            " - {} @ {} (function #{})",
                            new_station.s_type,
                            new_station.loc,
                            function_id
                        );
                        stations.push(new_station);
                        state = State::Default;
                    }
                    cur_token.clear();
                } else if c.is_ascii_graphic() && !c.is_ascii_whitespace() {
                    cur_token.push(c);
                } else {
                    // invalid character
                    return Err(Error::new(
                        SyntaxError,
                        pos,
                        "Invalid character, function names can only contain non-whitespace, printable ASCII characters",
                    ));
                }
            }
            State::FunctionSuffix(id) => {
                if c == ']' {
                    let new_station: Station;
                    let loc = SourceSpan::new(cur_station_pos, pos.col - cur_station_pos.col);
                    if cur_token == "out" {
                        new_station = Station {
                            loc,
                            s_type: &station::types::FUNC_OUTPUT,
                            modifiers: StationModifiers::default(),
                            in_bays: Vec::new(),
                            out_bays: Vec::new(),
                            data: StationData::FunctionID(id),
                        };
                        debug!(
                            3,
                            " - {} @ {} (function #{})", new_station.s_type, new_station.loc, id
                        );
                    } else if let Ok(index) = cur_token.parse::<usize>() {
                        new_station = Station {
                            loc,
                            s_type: &station::types::FUNC_INPUT,
                            modifiers: StationModifiers::default(),
                            in_bays: Vec::new(),
                            out_bays: Vec::new(),
                            data: StationData::FunctionIDAndIndex(id, index),
                        };
                        debug!(
                            3,
                            " - {} @ {} (function #{}, input {})",
                            new_station.s_type,
                            new_station.loc,
                            id,
                            index
                        );
                    } else {
                        return Err(Error::new(
                            SyntaxError,
                            loc,
                            "Invalid function suffix, must be 'out' or a positive integer",
                        ));
                    }
                    stations.push(new_station);
                    state = State::Default;
                } else if c.is_ascii_graphic() && !c.is_ascii_whitespace() {
                    cur_token.push(c);
                } else {
                    return Err(Error::new(SyntaxError, pos, "Invalid character"));
                }
            }
            State::StationModifiers(ref mods) => match c {
                'N' => state = State::StationModifiers(mods.with_priority(Direction::NORTH)),
                'E' => state = State::StationModifiers(mods.with_priority(Direction::EAST)),
                'S' => state = State::StationModifiers(mods.with_priority(Direction::SOUTH)),
                'W' => state = State::StationModifiers(mods.with_priority(Direction::WEST)),
                '*' => state = State::StationModifiers(mods.reverse()),
                ']' => {
                    let mut new_station = Station::from_str(
                        cur_token.as_str(),
                        SourceSpan::new(cur_station_pos, pos.col - cur_station_pos.col),
                    )?;
                    new_station.modifiers = *mods;
                    debug!(3, " - {} @ {}", new_station.s_type, new_station.loc);
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
                    // creating new station
                    let loc = SourceSpan::new(cur_station_pos, pos.col - cur_station_pos.col + 1);
                    let assign_val = match parse_assign_literal(&cur_token) {
                        Ok(p) => p,
                        Err(s) => {
                            return Err(Error::new(SyntaxError, pos, s));
                        }
                    };
                    let new_station = Station {
                        loc,
                        s_type: &station::types::ASSIGN,
                        modifiers: StationModifiers::default(),
                        in_bays: Vec::new(),
                        out_bays: Vec::new(),
                        data: StationData::AssignValue(assign_val.clone()),
                    };
                    debug!(
                        3,
                        " - {} @ {} ({})", new_station.s_type, new_station.loc, assign_val
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
                break;
            }
        };
    }
    debug!(3, "Functions:");
    for (name, id) in function_ids.iter() {
        debug!(3, " - {} (#{})", name, id);
    }
    match state {
        State::Default => {
            return Ok(stations);
        }
        _ => return Err(Error::new(SyntaxError, cur_station_pos, "Unexpected EOF")),
    }
}
