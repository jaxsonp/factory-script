use crate::*;

mod belt_follower;

#[cfg(test)]
mod tests;

/// Parses conveyor belts in the character map and connecting the stations accordingly
pub fn parse_conveyor_belts(
    char_map: &Vec<Vec<char>>,
    stations: &mut Vec<Station>,
) -> Result<(), Error> {
    let mut visited_map: Vec<Vec<bool>> = Vec::new();
    for line in char_map {
        visited_map.push(line.iter().map(|_| false).collect());
    }
    debug!(2, "Parsing conveyor belts");
    for i in 0..stations.len() {
        debug!(3, " - #{i} {}", stations[i].s_type.id);
        // get neighbors
        let neighbors = get_neighbors(char_map, &stations[i]);
        for neighbor in neighbors {
            // check if neighbors originate from a station
            if let Some(origin_pos) =
                belt_follower::follow_belt(char_map, &mut visited_map, neighbor)?
            {
                if let Some(origin_i) = get_station_at(stations, origin_pos) {
                    debug!(3, "   - bay {} from #{origin_i}", stations[i].in_bays.len());
                    let in_bay_index = stations[i].in_bays.len();
                    stations[i].in_bays.push(None);
                    stations[origin_i].out_bays.push((i, in_bay_index));
                } else {
                    return Err(Error::new(
                        SyntaxError,
                        origin_pos,
                        "Expected station at start of conveyor belt",
                    ));
                }
            };
        }
    }

    for line in 0..char_map.len() {
        for col in 0..char_map[line].len() {
            if BELT_CHARS.contains(char_map[line][col]) && !visited_map[line][col] {
                return Err(Error::new(
                    SyntaxError,
                    SourcePos::new(line, col),
                    "Unattached conveyor belt",
                ));
            }
        }
    }

    return Ok(());
}

/// Returns the station located at the specified position, if there is one
pub fn get_station_at(stations: &Vec<Station>, pos: SourcePos) -> Option<usize> {
    for i in 0..stations.len() {
        if stations[i].loc.pos.line == pos.line
            && stations[i].loc.pos.col <= pos.col
            && stations[i].loc.pos.col + stations[i].loc.len > pos.col
        {
            return Some(i);
        }
    }
    return None;
}

/// Gets the neighboring location of a specific station in order of highest priority
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
