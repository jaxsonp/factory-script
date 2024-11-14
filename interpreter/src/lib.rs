pub static mut COLOR_OUTPUT: bool = false;
pub static mut DEBUG_LEVEL: u8 = 0;

use std::{cmp::min, time::Instant};

pub mod error;
pub mod fs_core;
pub mod macros;
pub mod preprocessor;
pub mod runtime;

use error::{Error, ErrorType::*};
use fs_core::{stations::STATION_TYPES, *};

pub fn run(src: &str, print_benchmark: bool) -> Result<(), Error> {
    let start_time = Instant::now();

    debug!(2, "Preprocessing...");
    let (mut stations, start_i, assign_table) = preprocessor::process(src)?;
    let runtime_start_time = Instant::now();
    debug!(2, "Starting");
    let step_count = runtime::execute(&mut stations, start_i, &assign_table)?;

    if print_benchmark {
        let end_time = Instant::now();
        let preprocess_duration: f64 =
            ((runtime_start_time - start_time).as_nanos() as f64) / 1000000000f64;
        let runtime_duration: f64 =
            ((end_time - runtime_start_time).as_nanos() as f64) / 1000000000f64;
        let total_duration: f64 = ((end_time - start_time).as_nanos() as f64) / 1000000000f64;

        let avg_step_duration =
            ((end_time - runtime_start_time).as_nanos() as f64) / ((1000 * step_count) as f64);

        println!("\n======Benchmark======");
        println!(" steps      {}", step_count);
        println!(" avg step   {:.2}ms", avg_step_duration);
        println!();
        println!(" preprocess {:.5}s", preprocess_duration);
        println!(" runtime    {:.5}s", runtime_duration);
        println!(" total      {:.5}s", total_duration);
        println!("=====================");
    }

    Ok(())
}

/// Instance of a station
#[derive(Debug)]
pub struct Station {
    /// Location of the station in source code
    pub loc: SourceSpan,
    /// Station functionality and type information
    pub logic: &'static StationType,
    /// Modifiers duh
    pub modifiers: StationModifiers,
    /// Queues for each input bay
    pub in_bays: Vec<Option<Pallet>>,
    /// Map of each output bay connection in the form (station_index, in_bay_index)
    pub out_bays: Vec<(usize, usize)>,
}
impl Station {
    pub fn new(
        identifier: &str,
        loc: SourceSpan,
        modifiers: StationModifiers,
    ) -> Result<Self, Error> {
        for station_type in STATION_TYPES.iter() {
            if station_type.has_id(identifier) {
                return Ok(Self {
                    loc,
                    logic: station_type,
                    modifiers,
                    in_bays: Vec::new(),
                    out_bays: Vec::new(),
                });
            }
        }
        return Err(Error::new(
            IdentifierError,
            loc,
            format!("Failed to find station type with identifier \"{identifier}\"").as_str(),
        ));
    }

    pub fn clear_in_bays(&mut self) {
        for bay in self.in_bays.iter_mut() {
            if bay.is_some() {
                *bay = None;
            }
        }
    }
}

/// Struct for holding the modifiers of an instance of a station
#[derive(Debug, Clone, Copy)]
pub struct StationModifiers {
    /// Reverse input precedence (false=cw, true=ccw)
    pub reverse: bool,
    /// Which direction the precedence starts with
    pub priority: Direction,
}
impl StationModifiers {
    /// Default modifiers for a station
    pub fn default() -> Self {
        Self {
            reverse: false,
            priority: Direction::NORTH,
        }
    }
    /// toggles the reverse direction modifier
    pub fn reverse(self) -> Self {
        Self {
            reverse: !self.reverse,
            ..self
        }
    }
    /// sets the direction with priority to a new value
    pub fn with_priority(self, new_priority: Direction) -> Self {
        Self {
            priority: new_priority,
            ..self
        }
    }
}

/// Trait to describe a type that references a location in the source code

/// Defines the position of a span of characters in the source code, used for
/// syntax parsing and error reporting
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SourcePos {
    /// line number
    pub line: usize,
    /// column number
    pub col: usize,
}
impl SourcePos {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
    pub fn zero() -> Self {
        Self { line: 0, col: 0 }
    }
    pub fn spanning(&self, len: usize) -> SourceSpan {
        SourceSpan::new(*self, len)
    }
}
impl Into<SourceSpan> for SourcePos {
    fn into(self) -> SourceSpan {
        SourceSpan::new(self, 1)
    }
}
impl std::fmt::Display for SourcePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.col,)
    }
}

/// Defines the position of a span of characters in the source code, used for
/// syntax parsing and error reporting
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SourceSpan {
    /// line number
    pub pos: SourcePos,
    /// length of span
    pub len: usize,
}
impl SourceSpan {
    /// Value to represent if the source location is not applicable
    pub fn new(pos: SourcePos, len: usize) -> Self {
        Self { pos, len }
    }
    pub fn zero() -> Self {
        Self {
            pos: SourcePos::zero(),
            len: 0,
        }
    }
}
impl std::fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len > 1 {
            write!(f, "{}-{}", self.pos, self.pos.col + self.len)
        } else {
            write!(f, "{}", self.pos)
        }
    }
}

/// Helper for the cardinal directions
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
impl std::ops::Not for Direction {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::EAST => Direction::WEST,
            Direction::SOUTH => Direction::NORTH,
            Direction::WEST => Direction::EAST,
        }
    }
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::NORTH => "north",
                Direction::EAST => "east",
                Direction::SOUTH => "south",
                Direction::WEST => "west",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_negation() {
        assert_eq!(!Direction::NORTH, Direction::SOUTH);
        assert_eq!(!Direction::EAST, Direction::WEST);
        assert_eq!(!Direction::SOUTH, Direction::NORTH);
        assert_eq!(!Direction::WEST, Direction::EAST);
    }

    #[test]
    fn test_station_clear_in_bays() {
        let mut station = Station::new(
            "joint",
            SourcePos::zero().spanning(0),
            StationModifiers::default(),
        )
        .unwrap();
        station.in_bays.push(Some(Pallet::Empty));
        station.in_bays.push(Some(Pallet::Int(3)));
        station.in_bays.push(Some(Pallet::Char('a')));
        station.clear_in_bays();
        assert!(station.in_bays[0].is_none());
        assert!(station.in_bays[1].is_none());
        assert!(station.in_bays[2].is_none());
    }

    #[test]
    fn test_station_modifiers() {
        assert!(matches!(
            StationModifiers::default(),
            StationModifiers {
                reverse: false,
                priority: Direction::NORTH
            }
        ));
        assert!(matches!(
            StationModifiers::default().reverse(),
            StationModifiers {
                reverse: true,
                priority: Direction::NORTH
            }
        ));
        assert!(matches!(
            StationModifiers::default().with_priority(Direction::SOUTH),
            StationModifiers {
                reverse: false,
                priority: Direction::SOUTH
            }
        ));
        assert!(matches!(
            StationModifiers::default()
                .reverse()
                .with_priority(Direction::EAST),
            StationModifiers {
                reverse: true,
                priority: Direction::EAST
            }
        ));
    }
}
