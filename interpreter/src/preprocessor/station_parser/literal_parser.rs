use crate::*;

pub fn parse_assign_literal(s: &String) -> Result<Pallet, String> {
    match s.as_str() {
        "" => {
            // empty pallet
            return Ok(Pallet::Empty);
        }
        "true" => {
            // boolean true
            return Ok(Pallet::Bool(true));
        }
        "false" => {
            // boolean false
            return Ok(Pallet::Bool(false));
        }
        "pi" => return Ok(pallet::constants::PI.clone()),
        "e" => return Ok(pallet::constants::E.clone()),
        _ => {}
    }

    if s.starts_with('"') {
        // string literal
        if !s.ends_with('"') {
            return Err("Unclosed string literal".to_string());
        }
        let string = &s[1..(s.len() - 1)];
        return Ok(Pallet::String(string.to_owned()));
    } else if s.starts_with('\'') {
        // char literal
        if !s.ends_with('\'') {
            return Err("Unclosed character literal".to_string());
        }
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 3 {
            return Err("Malformed character literal".to_string());
        }
        return Ok(Pallet::Char(chars[1]));
    }

    let mut parsed_string = String::new();
    let mut decimal = false;
    let mut float_terminal = false;
    for c in s.chars().collect::<Vec<char>>() {
        if float_terminal {
            // f character already found
            return Err("Unexpected character(s) after float literal".to_string());
        }
        match c {
            '_' => {
                // ignoring underscores
                continue;
            }
            '.' => {
                // checking for double decimal points
                if decimal {
                    return Err(
                        "Malformed float literal, found multiple decimal points".to_string()
                    );
                }
                decimal = true;
                parsed_string.push('.');
            }
            'f' => {
                float_terminal = true;
            }
            c if c.is_ascii_digit() => parsed_string.push(c),
            _ => {
                return Err("Invalid assignment literal".to_string());
            }
        }
    }

    if !decimal && !float_terminal {
        // integer literal
        match i64::from_str_radix(parsed_string.as_str(), 10) {
            Ok(num) => return Ok(Pallet::Int(num)),
            Err(e) => {
                return Err(format!("Failed to parse integer literal ({e})"));
            }
        };
    } else {
        // float literal
        match parsed_string.parse::<f64>() {
            Ok(num) => return Ok(Pallet::Float(num)),
            Err(e) => {
                return Err(format!("Failed to parse float literal ({e})"));
            }
        };
    }
}
