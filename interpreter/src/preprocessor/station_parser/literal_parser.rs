use crate::{util::SourceSpan, *};

pub fn parse_assign_literal(s: &String, loc: SourceSpan) -> Result<Pallet, Error> {
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
            return Err(Error::new(SyntaxError, loc, "Unclosed string literal"));
        }
        let string = &s[1..(s.len() - 1)];
        return Ok(Pallet::String(string.to_owned()));
    } else if s.starts_with('\'') {
        // char literal
        if !s.ends_with('\'') {
            return Err(Error::new(SyntaxError, loc, "Unclosed character literal"));
        }
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 3 {
            return Err(Error::new(SyntaxError, loc, "Malformed character literal"));
        }
        return Ok(Pallet::Char(chars[1]));
    }

    let mut parsed_string = String::new();
    let mut decimal = false;
    let mut float_terminal = false;
    for c in s.chars().collect::<Vec<char>>() {
        if float_terminal {
            // f character already found
            return Err(Error::new(
                SyntaxError,
                loc,
                "Unexpected character(s) after float literal",
            ));
        }
        match c {
            '_' => {
                // ignoring underscores
                continue;
            }
            '.' => {
                // checking for double decimal points
                if decimal {
                    return Err(Error::new(
                        SyntaxError,
                        loc,
                        "Malformed float literal, found multiple decimal points",
                    ));
                }
                decimal = true;
                parsed_string.push('.');
            }
            'f' => {
                float_terminal = true;
            }
            c if c.is_ascii_digit() => parsed_string.push(c),
            _ => {
                return Err(Error::new(SyntaxError, loc, "Invalid assignment literal"));
            }
        }
    }

    if !decimal && !float_terminal {
        // integer literal
        match i64::from_str_radix(parsed_string.as_str(), 10) {
            Ok(num) => return Ok(Pallet::Int(num)),
            Err(e) => {
                return Err(Error::new(
                    SyntaxError,
                    loc,
                    format!("Failed to parse integer literal ({e})"),
                ));
            }
        };
    } else {
        // float literal
        match parsed_string.parse::<f64>() {
            Ok(num) => return Ok(Pallet::Float(num)),
            Err(e) => {
                return Err(Error::new(
                    SyntaxError,
                    loc,
                    format!("Failed to parse float literal ({e})"),
                ));
            }
        };
    }
}
