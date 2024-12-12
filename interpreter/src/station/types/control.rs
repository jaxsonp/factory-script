use super::*;

pub static MAIN: StationType = StationType {
    id: "main",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: main_procedure,
};
fn main_procedure(_: Vec<Pallet>) -> Result<Option<Pallet>, String> {
    return Ok(Some(Pallet::Empty));
}

pub static EXIT: StationType = StationType {
    id: "exit",
    alt_id: None,
    inputs: 1,
    output: false,
    procedure: none_procedure,
};

pub static JOINT: StationType = StationType {
    id: "joint",
    alt_id: Some(""),
    inputs: 1,
    output: true,
    procedure: joint_procedure,
};
fn joint_procedure(pallets: Vec<Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 1, "Invalid argument count");
    return Ok(Some(pallets[0].clone()));
}

pub static ASSIGN: StationType = StationType {
    id: "assign",
    alt_id: None,
    inputs: 1,
    output: true,
    procedure: none_procedure,
};

pub static GATE: StationType = StationType {
    id: "gate",
    alt_id: None,
    inputs: 2,
    output: true,
    procedure: gate_procedure,
};
fn gate_procedure(pallets: Vec<Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Bool(b), pallet) | (pallet, Pallet::Bool(b)) => {
            Ok(if *b { Some((*pallet).clone()) } else { None })
        }
        _ => {
            return Err(format!(
                "Expected at least one boolean pallet, received {}\n",
                list_pallets(&pallets)
            ));
        }
    }
}

pub static FILTER: StationType = StationType {
    id: "filter",
    alt_id: Some("X"),
    inputs: 1,
    output: true,
    procedure: filter_procedure,
};
fn filter_procedure(pallets: Vec<Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 1, "Invalid argument count");
    match &pallets[0] {
        Pallet::Bool(false) => Ok(None),
        p => Ok(Some((*p).clone())),
    }
}
