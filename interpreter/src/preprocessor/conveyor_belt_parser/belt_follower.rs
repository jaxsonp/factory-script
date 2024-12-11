use crate::{preprocessor::*, util::*, *};

/// Given a starting position around a station, check if it is an input bay and
/// if it is find the origin of the conveyor belt
///
/// Returns an optional tuple of the origin position if it is an input bay
pub fn follow_belt(
    map: &Vec<Vec<char>>,
    visited_map: &mut Vec<Vec<bool>>,
    start: (SourcePos, Direction),
) -> Result<Option<SourcePos>, Error> {
    let mut pos = start.0;
    let mut facing = start.1;
    let mut c = map[pos.line][pos.col];
    let mut visited: Vec<SourcePos> = Vec::new();

    // checking if not a single belt character
    if !SINGLE_BELT_CHARS.contains(c) {
        return Ok(None);
    }

    // checking if pointing into station
    match facing {
        Direction::NORTH => {
            if !SOUTH_BELT_CHARS.contains(c) {
                return Ok(None);
            }
        }
        Direction::EAST => {
            if !WEST_BELT_CHARS.contains(c) {
                return Ok(None);
            }
        }
        Direction::SOUTH => {
            if !NORTH_BELT_CHARS.contains(c) {
                return Ok(None);
            }
        }
        Direction::WEST => {
            if !EAST_BELT_CHARS.contains(c) {
                return Ok(None);
            }
        }
    }

    debug!(4, "      probing starting at {}", pos);
    loop {
        if BELT_CHARS.contains(c) {
            visited.push(pos);
        }

        // checking if current char connects to previous char and turning
        if facing == Direction::NORTH && SOUTH_BELT_CHARS.contains(c) {
            match c {
                '│' | '║' => {}
                '┌' | '╔' => facing = Direction::EAST,
                '┐' | '╗' => facing = Direction::WEST,
                _ => panic!(),
            }
        } else if facing == Direction::EAST && WEST_BELT_CHARS.contains(c) {
            match c {
                '─' | '═' => {}
                '┘' | '╝' => facing = Direction::NORTH,
                '┐' | '╗' => facing = Direction::SOUTH,
                _ => panic!(),
            }
        } else if facing == Direction::SOUTH && NORTH_BELT_CHARS.contains(c) {
            match c {
                '│' | '║' => {}
                '└' | '╚' => facing = Direction::EAST,
                '┘' | '╝' => facing = Direction::WEST,
                _ => panic!(),
            }
        } else if facing == Direction::WEST && EAST_BELT_CHARS.contains(c) {
            match c {
                '─' | '═' => {}
                '└' | '╚' => facing = Direction::NORTH,
                '┌' | '╔' => facing = Direction::SOUTH,
                _ => panic!(),
            }
        } else {
            // dangling belt
            return Err(Error::new(
                SyntaxError,
                pos,
                "Dangling belt, expected station out bay",
            ));
        }
        debug!(4, "       - moved to {}, now facing {}", pos, facing);

        // moving to the next char
        match facing {
            Direction::NORTH => {
                if pos.line == 0 {
                    break;
                }
                pos.line -= 1;
            }
            Direction::EAST => {
                pos.col += 1;
                if pos.col >= map[pos.line].len() {
                    break;
                }
            }
            Direction::SOUTH => {
                pos.line += 1;
                if pos.line >= map.len() {
                    break;
                }
            }
            Direction::WEST => {
                if pos.col == 0 {
                    break;
                }
                pos.col -= 1;
            }
        }
        // if the last character was a double belt, we reached the origin
        if DOUBLE_BELT_CHARS.contains(c) {
            debug!(4, "       - path ended at {}", pos);
            for pos in visited {
                visited_map[pos.line][pos.col] = true;
            }
            return Ok(Some(pos));
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
