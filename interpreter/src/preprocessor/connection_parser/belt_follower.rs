use crate::{preprocessor::*, util::Direction::*, *};

/// Given a starting position around a station, check if it is an output bay and
/// if it is find the destination
///
/// Returns an optional tuple of the destination index and in bay priority if it is an output bay
pub fn follow_belt(
    map: &Vec<Vec<char>>,
    stations: &Vec<Station>,
    start: (SourcePos, Direction),
) -> Result<Option<(usize, u32)>, Error> {
    let mut pos = start.0;
    let mut last_pos = pos;
    let mut facing = start.1;
    let mut c = map[pos.line][pos.col];

    // Returns the station located at the specified position, if there is one
    let get_station_at = |pos: SourcePos| -> Option<usize> {
        for (i, station) in stations.iter().enumerate() {
            if station.loc.pos.line == pos.line
                && station.loc.pos.col <= pos.col
                && station.loc.pos.col + station.loc.len > pos.col
            {
                return Some(i);
            }
        }
        return None;
    };

    // checking if not a double belt character
    if !DOUBLE_BELT_CHARS.contains(c) {
        return Ok(None);
    }

    // checking if pointing into station
    if facing == NORTH && !SOUTH_BELT_CHARS.contains(c) {
        return Ok(None);
    } else if facing == EAST && !WEST_BELT_CHARS.contains(c) {
        return Ok(None);
    } else if facing == SOUTH && !NORTH_BELT_CHARS.contains(c) {
        return Ok(None);
    } else if facing == WEST && !EAST_BELT_CHARS.contains(c) {
        return Ok(None);
    }

    let mut belt_len: u32 = 0;
    loop {
        if !BELT_CHARS.contains(c) {
            if belt_len <= 1 {
                return Err(Error::new(SyntaxError, pos, "Invalid conveyor belt"));
            }
            match get_station_at(pos) {
                Some(dest) => {
                    // finding input priority of destination in bay
                    for (i, neighbor) in get_neighbors_inorder(map, &stations[dest])
                        .iter()
                        .enumerate()
                    {
                        if neighbor.0 == last_pos {
                            return Ok(Some((dest, i as u32)));
                        }
                    }
                    return Ok(None);
                }
                None => {
                    return Ok(None);
                }
            }
        }
        belt_len += 1;

        // checking if current char connects to previous char and turning
        if facing == NORTH && SOUTH_BELT_CHARS.contains(c) {
            match c {
                '│' | '║' => {}
                '┌' | '╔' => facing = EAST,
                '┐' | '╗' => facing = WEST,
                _ => panic!(),
            }
        } else if facing == EAST && WEST_BELT_CHARS.contains(c) {
            match c {
                '─' | '═' => {}
                '┘' | '╝' => facing = NORTH,
                '┐' | '╗' => facing = SOUTH,
                _ => panic!(),
            }
        } else if facing == SOUTH && NORTH_BELT_CHARS.contains(c) {
            match c {
                '│' | '║' => {}
                '└' | '╚' => facing = EAST,
                '┘' | '╝' => facing = WEST,
                _ => panic!(),
            }
        } else if facing == WEST && EAST_BELT_CHARS.contains(c) {
            match c {
                '─' | '═' => {}
                '└' | '╚' => facing = NORTH,
                '┌' | '╔' => facing = SOUTH,
                _ => panic!(),
            }
        } else {
            // dangling belt
            return Ok(None);
        }

        // moving to the next char
        last_pos = pos;
        match facing {
            NORTH => {
                if pos.line == 0 {
                    break;
                }
                pos.line -= 1;
            }
            EAST => {
                pos.col += 1;
                if pos.col >= map[pos.line].len() {
                    break;
                }
            }
            SOUTH => {
                pos.line += 1;
                if pos.line >= map.len() {
                    break;
                }
            }
            WEST => {
                if pos.col == 0 {
                    break;
                }
                pos.col -= 1;
            }
        }
        // moving
        c = map[pos.line][pos.col];
    }
    // dangling belt out of bounds
    return Err(Error::new(
        SyntaxError,
        SourceSpan::new(SourcePos::new(pos.line, pos.col), 1),
        "Unattached conveyor belt",
    ));
}

/// Gets the neighboring locations of a specific station in order of highest priority
pub fn get_neighbors_inorder(
    map: &Vec<Vec<char>>,
    station: &Station,
) -> Vec<(SourcePos, Direction)> {
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
