mod belt_follower;

use std::collections::HashMap;

use belt_follower::follow_belt;

use super::*;
use crate::{
    station::{self, StationData},
    *,
};

/// Parses conveyor belts in the character map, connects the stations, and moves them into their function templates
pub fn parse(
    char_map: &Vec<Vec<char>>,
    stations: Vec<Station>,
    functions: &mut Vec<FunctionTemplate>,
) -> Result<(), Error> {
    let mut stations = stations;
    // hashmap to keep track of which stations have been visited, and by what function id
    let mut visited_stations: HashMap<usize, usize> = HashMap::with_capacity(stations.len());

    // getting all station indices that are an entry point for a function and which function they belong to
    let mut entry_stations: Vec<(usize, usize)> = Vec::new();
    for (i, station) in stations.iter().enumerate() {
        if station.s_type == &station::types::MAIN {
            entry_stations.push((i, 0));
        } else if station.s_type == &station::types::FUNC_INPUT {
            if let StationData::FunctionIDAndIndex(id, _) = station.data {
                entry_stations.push((i, id));
            } else {
                panic!();
            }
        }
    }

    // Performing DFS starting at every entry station
    debug!(2, "Parsing connections");
    for (entry_index, cur_function_id) in entry_stations {
        let mut to_visit: Vec<usize> = Vec::new();
        to_visit.push(entry_index);

        while !to_visit.is_empty() {
            let i = to_visit.pop().unwrap();
            if visited_stations.contains_key(&i) {
                // station has been visited already
                if *visited_stations.get(&i).unwrap() != cur_function_id {
                    return Err(Error::new(
                        SyntaxError,
                        stations[i].loc,
                        format!(
                            "Station cannot belong to multiple function templates, found in functions '{}' and '{}",
                            functions[*visited_stations.get(&i).unwrap()].name,
                            functions[cur_function_id].name
                        ),
                    ));
                }
                continue;
            }
            // marking this station as being visited by this function
            visited_stations.insert(i, cur_function_id);

            //println!(" at {i}");
            let neighbors = get_neighbors(char_map, &stations[i]);
            for neighbor in neighbors {
                let (dest, priority) = match follow_belt(char_map, &stations, neighbor)? {
                    Some(res) => res,
                    None => {
                        // neighbor position isn't a conveyor belt
                        continue;
                    }
                };
                if stations[dest].s_type == &station::types::FUNC_OUTPUT {
                    if let StationData::FunctionID(id) = stations[dest].data {
                        if id != cur_function_id {
                            return Err(Error::new(
                                SyntaxError,
                                stations[dest].loc,
                                format!(
                                    "Found output for function '{}' when evaluating function '{}'",
                                    functions[id].name, functions[cur_function_id].name
                                ),
                            ));
                        }
                    }
                }
                stations[i].out_bays.push((dest, priority));
                //println!("   goes to {dest}");
                to_visit.push(dest);
            }
        }
    }

    // used to map old global indices of stations to function-local indices
    let mut index_mappings: HashMap<usize, usize> = HashMap::new();

    // moving every station into its proper function template
    for (i, function_id) in visited_stations {
        index_mappings.insert(i, functions[function_id].stations.len());
        functions[function_id].stations.push(stations[i].clone());
    }

    for f in functions.iter_mut() {
        debug!(4, "function '{}':", f.name);
        for (i, s) in f.stations.iter_mut().enumerate() {
            debug!(4, " - {i} {} @ {}", s.s_type, s.loc);
            for (dest, priority) in s.out_bays.iter_mut() {
                // updating connection indices
                if let Some(new_i) = index_mappings.get(dest) {
                    *dest = *new_i;
                }
                debug!(5, "    -> {dest} ({priority})");
            }
        }
    }

    return Ok(());
}

/// Gets the neighboring locations of a specific station in order of highest priority
pub fn get_neighbors(map: &Vec<Vec<char>>, station: &Station) -> Vec<(SourcePos, Direction)> {
    let mut neighbors: Vec<(SourcePos, Direction)> = Vec::new();

    let mut northern_neighbors: Vec<(SourcePos, Direction)> = Vec::new();
    if station.loc.pos.line > 0 {
        for i in 0..station.loc.len {
            if station.loc.pos.col + i < map[station.loc.pos.line - 1].len() {
                northern_neighbors.push((
                    SourcePos::new(station.loc.pos.line - 1, station.loc.pos.col + i),
                    Direction::NORTH,
                ));
            }
        }
    }
    let mut eastern_neighbors: Vec<(SourcePos, Direction)> = Vec::new();
    if station.loc.pos.col + station.loc.len < map[station.loc.pos.line].len() {
        eastern_neighbors.push((
            SourcePos::new(station.loc.pos.line, station.loc.pos.col + station.loc.len),
            Direction::EAST,
        ));
    }
    let mut southern_neighbors: Vec<(SourcePos, Direction)> = Vec::new();
    if station.loc.pos.line < (map.len() - 1) {
        for i in (0..station.loc.len).rev() {
            if station.loc.pos.col + i < map[station.loc.pos.line + 1].len() {
                southern_neighbors.push((
                    SourcePos::new(station.loc.pos.line + 1, station.loc.pos.col + i),
                    Direction::SOUTH,
                ));
            }
        }
    }
    let mut western_neighbors: Vec<(SourcePos, Direction)> = Vec::new();
    if station.loc.pos.col > 0 {
        western_neighbors.push((
            SourcePos::new(station.loc.pos.line, station.loc.pos.col - 1),
            Direction::WEST,
        ));
    }

    if !station.modifiers.reverse {
        // clockwise
        match station.modifiers.priority {
            Direction::NORTH => {
                neighbors.extend(northern_neighbors);
                neighbors.extend(eastern_neighbors);
                neighbors.extend(southern_neighbors);
                neighbors.extend(western_neighbors);
            }
            Direction::EAST => {
                neighbors.extend(eastern_neighbors);
                neighbors.extend(southern_neighbors);
                neighbors.extend(western_neighbors);
                neighbors.extend(northern_neighbors);
            }
            Direction::SOUTH => {
                neighbors.extend(southern_neighbors);
                neighbors.extend(western_neighbors);
                neighbors.extend(northern_neighbors);
                neighbors.extend(eastern_neighbors);
            }
            Direction::WEST => {
                neighbors.extend(western_neighbors);
                neighbors.extend(northern_neighbors);
                neighbors.extend(eastern_neighbors);
                neighbors.extend(southern_neighbors);
            }
        }
    } else {
        // counter clockwise
        match station.modifiers.priority {
            Direction::NORTH => {
                neighbors.extend(northern_neighbors.iter().rev());
                neighbors.extend(western_neighbors);
                neighbors.extend(southern_neighbors.iter().rev());
                neighbors.extend(eastern_neighbors);
            }
            Direction::EAST => {
                neighbors.extend(eastern_neighbors);
                neighbors.extend(northern_neighbors.iter().rev());
                neighbors.extend(western_neighbors);
                neighbors.extend(southern_neighbors.iter().rev());
            }
            Direction::SOUTH => {
                neighbors.extend(southern_neighbors.iter().rev());
                neighbors.extend(eastern_neighbors);
                neighbors.extend(northern_neighbors.iter().rev());
                neighbors.extend(western_neighbors);
            }
            Direction::WEST => {
                neighbors.extend(western_neighbors);
                neighbors.extend(southern_neighbors.iter().rev());
                neighbors.extend(eastern_neighbors);
                neighbors.extend(northern_neighbors.iter().rev());
            }
        }
    }
    return neighbors;
}
