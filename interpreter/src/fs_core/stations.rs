use crate::*;
use lazy_static::lazy_static;

pub mod control;
pub mod io;
pub mod math;

lazy_static! {
    pub static ref STATION_TYPES: Vec<&'static StationType> = vec![
        &control::START,
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
pub fn none_procedure(_: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    return Ok(None);
}

/// helper function to generate a string listing pallets, used for error messages
fn list_pallets(pallets: &Vec<Option<Pallet>>) -> String {
    let mut output = String::from("(");
    for i in 0..pallets.len() {
        output.push_str(
            match &pallets[i] {
                Some(pallet) => format!("{pallet}"),
                None => String::from("None"),
            }
            .as_str(),
        );
        if i != pallets.len() - 1 {
            output.push_str(", ")
        }
    }
    output.push(')');
    return output;
}
