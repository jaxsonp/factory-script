use super::*;

pub static EQUALS: BuiltinStationType = BuiltinStationType {
    id: "eq",
    alt_id: Some("="),
    inputs: 2,
    output: true,
    procedure: equals_procedure,
};
fn equals_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    return Ok(Some(Pallet::Bool(pallets[0] == pallets[1])));
}

pub static NOT_EQUALS: BuiltinStationType = BuiltinStationType {
    id: "ne",
    alt_id: Some("!="),
    inputs: 2,
    output: true,
    procedure: not_equals_procedure,
};
fn not_equals_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    return Ok(Some(Pallet::Bool(pallets[0] != pallets[1])));
}

pub static GREATER_THAN: BuiltinStationType = BuiltinStationType {
    id: "gt",
    alt_id: Some(">"),
    inputs: 2,
    output: true,
    procedure: greater_than_procedure,
};
fn greater_than_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Int(num1), Pallet::Int(num2)) => {
            return Ok(Some(Pallet::Bool(num1 > num2)));
        }
        (Pallet::Float(num1), Pallet::Float(num2)) => {
            return Ok(Some(Pallet::Bool(num1 > num2)));
        }
        (Pallet::Bool(bool1), Pallet::Bool(bool2)) => {
            return Ok(Some(Pallet::Bool(bool1 > bool2)));
        }
        _ => {
            return Err(format!(
                "Expected matching numerical or boolean pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static LESS_THAN: BuiltinStationType = BuiltinStationType {
    id: "lt",
    alt_id: Some("<"),
    inputs: 2,
    output: true,
    procedure: less_than_procedure,
};
fn less_than_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Int(num1), Pallet::Int(num2)) => {
            return Ok(Some(Pallet::Bool(num1 < num2)));
        }
        (Pallet::Float(num1), Pallet::Float(num2)) => {
            return Ok(Some(Pallet::Bool(num1 < num2)));
        }
        (Pallet::Bool(bool1), Pallet::Bool(bool2)) => {
            return Ok(Some(Pallet::Bool(bool1 < bool2)));
        }
        _ => {
            return Err(format!(
                "Expected matching numerical or boolean pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static GREATER_THAN_EQUAL: BuiltinStationType = BuiltinStationType {
    id: "gte",
    alt_id: Some(">="),
    inputs: 2,
    output: true,
    procedure: greater_than_equal_procedure,
};
fn greater_than_equal_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Int(num1), Pallet::Int(num2)) => {
            return Ok(Some(Pallet::Bool(num1 >= num2)));
        }
        (Pallet::Float(num1), Pallet::Float(num2)) => {
            return Ok(Some(Pallet::Bool(num1 >= num2)));
        }
        (Pallet::Bool(bool1), Pallet::Bool(bool2)) => {
            return Ok(Some(Pallet::Bool(bool1 >= bool2)));
        }
        _ => {
            return Err(format!(
                "Expected matching numerical or boolean pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static LESS_THAN_EQUAL: BuiltinStationType = BuiltinStationType {
    id: "lte",
    alt_id: Some("<="),
    inputs: 2,
    output: true,
    procedure: less_than_equal_procedure,
};
fn less_than_equal_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Int(num1), Pallet::Int(num2)) => {
            return Ok(Some(Pallet::Bool(num1 <= num2)));
        }
        (Pallet::Float(num1), Pallet::Float(num2)) => {
            return Ok(Some(Pallet::Bool(num1 <= num2)));
        }
        (Pallet::Bool(bool1), Pallet::Bool(bool2)) => {
            return Ok(Some(Pallet::Bool(bool1 <= bool2)));
        }
        _ => {
            return Err(format!(
                "Expected matching numerical or boolean pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static ADD: BuiltinStationType = BuiltinStationType {
    id: "add",
    alt_id: Some("+"),
    inputs: 2,
    output: true,
    procedure: add_procedure,
};
fn add_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Int(num1), Pallet::Int(num2)) => {
            return Ok(Some(Pallet::Int(num1 + num2)));
        }
        (Pallet::Float(num1), Pallet::Float(num2)) => {
            return Ok(Some(Pallet::Float(num1 + num2)));
        }
        (Pallet::String(string), Pallet::Char(char)) => {
            let mut s = string.to_owned();
            s.push(*char);
            return Ok(Some(Pallet::String(s)));
        }
        (Pallet::String(string1), Pallet::String(string2)) => {
            return Ok(Some(Pallet::String(string1.to_owned() + string2.as_str())));
        }
        _ => {
            return Err(format!(
                "Unexpected pallet types received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static SUBTRACT: BuiltinStationType = BuiltinStationType {
    id: "sub",
    alt_id: Some("-"),
    inputs: 2,
    output: true,
    procedure: subtract_procedure,
};
fn subtract_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Int(num1), Pallet::Int(num2)) => {
            return Ok(Some(Pallet::Int(num1 - num2)));
        }
        (Pallet::Float(num1), Pallet::Float(num2)) => {
            return Ok(Some(Pallet::Float(num1 - num2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static MULTIPLY: BuiltinStationType = BuiltinStationType {
    id: "mult",
    alt_id: Some("*"),
    inputs: 2,
    output: true,
    procedure: multiply_procedure,
};
fn multiply_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Int(num1), Pallet::Int(num2)) => {
            return Ok(Some(Pallet::Int(num1 * num2)));
        }
        (Pallet::Float(num1), Pallet::Float(num2)) => {
            return Ok(Some(Pallet::Float(num1 * num2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static DIVIDE: BuiltinStationType = BuiltinStationType {
    id: "div",
    alt_id: Some("/"),
    inputs: 2,
    output: true,
    procedure: divide_procedure,
};
fn divide_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Int(num1), Pallet::Int(num2)) => {
            if *num2 == 0 {
                return Err(String::from("Attempted divide by zero"));
            }
            return Ok(Some(Pallet::Int(num1 / num2)));
        }
        (Pallet::Float(num1), Pallet::Float(num2)) => {
            if *num2 == 0.0 {
                return Err(String::from("Attempted divide by zero"));
            }
            return Ok(Some(Pallet::Float(num1 / num2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static MODULO: BuiltinStationType = BuiltinStationType {
    id: "mod",
    alt_id: Some("%"),
    inputs: 2,
    output: true,
    procedure: modulo_procedure,
};
fn modulo_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Int(num1), Pallet::Int(num2)) => {
            if *num2 == 0 {
                return Err(String::from("Attempted divide by zero"));
            }
            return Ok(Some(Pallet::Int(num1 % num2)));
        }
        (Pallet::Float(num1), Pallet::Float(num2)) => {
            if *num2 == 0.0 {
                return Err(String::from("Attempted divide by zero"));
            }
            return Ok(Some(Pallet::Float(num1 % num2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static INCREMENT: BuiltinStationType = BuiltinStationType {
    id: "inc",
    alt_id: Some("++"),
    inputs: 1,
    output: true,
    procedure: increment_procedure,
};
fn increment_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 1, "Invalid argument count");
    match &pallets[0] {
        Pallet::Int(num) => {
            return Ok(Some(Pallet::Int(num + 1)));
        }
        Pallet::Float(num) => {
            return Ok(Some(Pallet::Float(num + 1.0)));
        }
        _ => {
            return Err(format!(
                "Expected a numerical pallet, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static DECREMENT: BuiltinStationType = BuiltinStationType {
    id: "dec",
    alt_id: Some("--"),
    inputs: 1,
    output: true,
    procedure: decrement_procedure,
};
fn decrement_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 1, "Invalid argument count");
    match &pallets[0] {
        Pallet::Int(num) => {
            return Ok(Some(Pallet::Int(num - 1)));
        }
        Pallet::Float(num) => {
            return Ok(Some(Pallet::Float(num - 1.0)));
        }
        _ => {
            return Err(format!(
                "Expected a numerical pallet, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static AND: BuiltinStationType = BuiltinStationType {
    id: "and",
    alt_id: None,
    inputs: 2,
    output: true,
    procedure: and_procedure,
};
fn and_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Bool(b1), Pallet::Bool(b2)) => Ok(Some(Pallet::Bool(*b1 && *b2))),
        _ => {
            return Err(format!(
                "Expected two boolean pallets, received {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static OR: BuiltinStationType = BuiltinStationType {
    id: "or",
    alt_id: None,
    inputs: 2,
    output: true,
    procedure: or_procedure,
};
fn or_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 2, "Invalid argument count");
    match (&pallets[0], &pallets[1]) {
        (Pallet::Bool(b1), Pallet::Bool(b2)) => Ok(Some(Pallet::Bool(*b1 || *b2))),
        _ => {
            return Err(format!(
                "Expected two boolean pallets, received {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static NOT: BuiltinStationType = BuiltinStationType {
    id: "not",
    alt_id: Some("!"),
    inputs: 1,
    output: true,
    procedure: not_procedure,
};
fn not_procedure(pallets: Vec<&Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 1, "Invalid argument count");
    match &pallets[0] {
        Pallet::Bool(b1) => Ok(Some(Pallet::Bool(!(*b1)))),
        _ => {
            return Err(format!(
                "Expected a boolean pallet, received {}\n",
                list_pallets(pallets)
            ));
        }
    }
}
