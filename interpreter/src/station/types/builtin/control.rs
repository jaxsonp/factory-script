use super::*;

pub static START: BuiltinStationType = BuiltinStationType {
    id: "start",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: start_procedure,
};
fn start_procedure(_: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    return Ok(Some(Pallet::Empty));
}

pub static EXIT: BuiltinStationType = BuiltinStationType {
    id: "exit",
    alt_id: None,
    inputs: 1,
    output: false,
    procedure: none_procedure,
};

pub static JOINT: BuiltinStationType = BuiltinStationType {
    id: "joint",
    alt_id: Some(""),
    inputs: 1,
    output: true,
    procedure: none_procedure,
};

pub static ASSIGN: BuiltinStationType = BuiltinStationType {
    id: "assign",
    alt_id: None,
    inputs: 1,
    output: true,
    procedure: none_procedure,
};

pub static GATE: BuiltinStationType = BuiltinStationType {
    id: "gate",
    alt_id: None,
    inputs: 2,
    output: true,
    procedure: gate_procedure,
};
fn gate_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Bool(b), pallet) | (pallet, Pallet::Bool(b)) => {
            Ok(if *b { Some((*pallet).clone()) } else { None })
        }
        _ => {
            return Err(format!(
                "Expected at least one boolean pallet, received {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static FILTER: BuiltinStationType = BuiltinStationType {
    id: "filter",
    alt_id: Some("X"),
    inputs: 1,
    output: true,
    procedure: filter_procedure,
};
fn filter_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match &pallets[0] {
        Pallet::Bool(false) => Ok(None),
        p => Ok(Some((*p).clone())),
    }
}
