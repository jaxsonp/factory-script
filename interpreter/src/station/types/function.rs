use super::*;

pub static FUNC_INVOKE: StationType = StationType {
    id: "func_invoke",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: none_procedure,
};

pub static FUNC_INPUT: StationType = StationType {
    id: "func_input",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: none_procedure,
};

pub static FUNC_OUTPUT: StationType = StationType {
    id: "func_output",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: none_procedure,
};
