use lazy_static::lazy_static;
use std::fmt;

mod control;
mod function;
mod io;
mod math;

pub use control::*;
pub use function::*;
pub use io::*;
pub use math::*;

use crate::Pallet;

/// Defines a builtin station and all the required information and functionality
#[derive(Debug, PartialEq)]
pub struct StationType {
    /// Identifier
    pub id: &'static str,
    /// Alternate identifier
    pub alt_id: Option<&'static str>,
    /// Minimum number of inputs required for this station to trigger its procedure
    pub inputs: usize,
    /// Does this station produce an output pallet
    pub output: bool,
    /// Station's procedure, takes a vector of input pallets and returns an optional
    /// pallet if successful, and an error message in a String if not
    pub procedure: fn(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String>,
}
impl StationType {
    /// Function to check whether a station has a certain ID
    pub fn has_id(&self, query: &str) -> bool {
        return self.id == query || (self.alt_id.is_some_and(|alt_id| alt_id == query));
    }
}
impl fmt::Display for StationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id,)
    }
}

lazy_static! {
    /// A manifest of all built in station types
    pub static ref STATION_TYPES: Vec<&'static StationType> = vec![
        &control::MAIN,
        &control::EXIT,
        &control::JOINT,
        &control::ASSIGN,
        &control::GATE,
        &control::FILTER,
        &io::PRINT,
        &io::PRINTLN,
        &io::READLN,
        &math::ADD,
        &math::SUBTRACT,
        &math::MULTIPLY,
        &math::DIVIDE,
        &math::MODULO,
        &math::EQUALS,
        &math::NOT_EQUALS,
        &math::GREATER_THAN,
        &math::LESS_THAN,
        &math::GREATER_THAN_EQUAL,
        &math::LESS_THAN_EQUAL,
        &math::INCREMENT,
        &math::DECREMENT,
        &math::AND,
        &math::NOT,
        &math::OR,
    ];
}

/// Common procedure that returns nothign
pub fn none_procedure(_: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    return Ok(None);
}

/// helper function to generate a string listing pallets, used for error messages
fn list_pallets(pallets: Vec<&Pallet>) -> String {
    let mut output = String::from("(");
    for i in 0..pallets.len() {
        output.push_str(format!("{}", pallets[i]).as_str());
        if i != pallets.len() - 1 {
            output.push_str(", ")
        }
    }
    output.push(')');
    return output;
}
