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

// for easy on/off debug statements
#[macro_export]
macro_rules! debug {
	($level:literal, $msg:literal, $($args:expr),*) => {
		unsafe {
			use crate::{COLOR_OUTPUT, DEBUG_LEVEL};
			if $level <= DEBUG_LEVEL {
				if $level > 1 && COLOR_OUTPUT {
					print!("\x1b[90m");
					print!($msg, $($args),*);
					println!("\x1b[0m");
				} else {
					println!($msg, $($args),*);
				}
			}
		}
    };
    ($level:literal, $msg:literal) => {
		unsafe {
			use crate::{COLOR_OUTPUT, DEBUG_LEVEL};
			if $level <= DEBUG_LEVEL {
				if $level > 1 && COLOR_OUTPUT {
					print!("\x1b[90m");
					print!($msg);
					println!("\x1b[0m");
				} else {
					println!($msg);
				}
			}
		}
    };
}
//pub(crate) use debug;

#[macro_export]
macro_rules! print_cli_err {
    ($msg:literal, $($args:expr),*) => {
        unsafe {
            if COLOR_OUTPUT {
                print!("\x1b[31m");
                print!($msg, $($args),*);
                println!("\x1b[0m");
            } else {
                print!("ERROR! ");
                println!($msg, $($args),*);
            }
        }
    };
    ($msg:literal) => {
        unsafe {
            if COLOR_OUTPUT {
                print!("\x1b[31m");
                print!($msg);
                println!("\x1b[0m");
            } else {
                print!("ERROR! ");
                println!($msg);
            }
        }
    };
}
//pub(crate) use print_cli_err;
