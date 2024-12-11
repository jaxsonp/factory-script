use super::none_procedure;
use crate::*;

// dummy station types used for parsing functions

pub static FUNC_INVOKE: BuiltinBuiltinStationType = BuiltinBuiltinStationType {
    id: "func_invoke",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: none_procedure,
};

pub static FUNC_INPUT: BuiltinBuiltinStationType = BuiltinBuiltinStationType {
    id: "func_input",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: none_procedure,
};

pub static FUNC_OUTPUT: BuiltinBuiltinStationType = BuiltinBuiltinStationType {
    id: "func_ouput",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: none_procedure,
};
