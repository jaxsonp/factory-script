use super::none_procedure;
use crate::*;

// dummy station types used for parsing functions

pub static FUNC_INVOKE: BuiltinStationType = BuiltinStationType {
    id: "func_invoke",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: none_procedure,
};

pub static FUNC_INPUT: BuiltinStationType = BuiltinStationType {
    id: "func_input",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: none_procedure,
};

pub static FUNC_OUTPUT: BuiltinStationType = BuiltinStationType {
    id: "func_ouput",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: none_procedure,
};
