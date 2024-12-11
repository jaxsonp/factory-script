use std::cmp::min;

use crate::util::SourceSpan;

/// Custom error handling struct
#[derive(Debug)]
pub struct Error {
    /// Error type
    ///
    /// named `t` cus `"type"` is a reserved keyword :_(
    pub t: ErrorType,
    /// Location the error originated from
    pub loc: SourceSpan,
    /// Message
    pub msg: String,
}

impl Error {
    pub fn new<Span: Into<SourceSpan>, Str: Into<String>>(
        t: ErrorType,
        loc: Span,
        msg: Str,
    ) -> Self {
        Self {
            t,
            loc: loc.into(),
            msg: msg.into(),
        }
    }

    /// function for generating a pretty error message
    pub fn pretty_msg(&self, src: &String) -> String {
        // don't print source location if location is zero
        if self.loc.len == 0 {
            return format!("{}: {}", self.t, self.msg);
        }

        // generating 2d vector layout of source code
        let mut char_map: Vec<Vec<char>> = Vec::new();
        for line in src.split('\n').collect::<Vec<&str>>() {
            char_map.push(line.chars().collect());
        }
        let mut output = format!("{} @ {}\n        ", self.t, self.loc);
        let left_bound = self.loc.pos.col.saturating_sub(24);
        let right_bound = min(80, self.loc.pos.col + self.loc.len + 24);

        for _ in left_bound..self.loc.pos.col {
            output += " ";
        }
        for _ in 0..self.loc.len {
            output += "v";
        }

        // closure to try and get a line of source code to print given an offset
        let try_get_ln = |offset: i32| -> String {
            let line = (self.loc.pos.line as i32) + offset;
            if line < 0 || line as usize >= char_map.len() {
                return String::new();
            }
            let line = line as usize;
            let left_bound = min(left_bound, char_map[line].len().saturating_sub(1));
            let right_bound = min(right_bound, char_map[line].len());
            let mut output = format!("\n\x1b[22m {:>4} | \x1b[2m", line + 1);
            for c in char_map[line][left_bound..right_bound].iter() {
                output.push(*c);
            }
            return output;
        };

        // printing lines above
        output += try_get_ln(-2).as_str();
        output += try_get_ln(-1).as_str();
        // printing line of error
        {
            let left_bound = min(
                left_bound,
                char_map[self.loc.pos.line].len().saturating_sub(1),
            );
            let right_bound = min(right_bound, char_map[self.loc.pos.line].len());
            output += format!("\n\x1b[22m-{:->4}-| \x1b[2m", self.loc.pos.line + 1).as_str();
            for c in char_map[self.loc.pos.line][left_bound..self.loc.pos.col].iter() {
                output.push(*c);
            }
            // bold and underline
            output += "\x1b[22m\x1b[1m\x1b[4m";
            let highlighted_right_bound = min(
                self.loc.pos.col + self.loc.len,
                char_map[self.loc.pos.line].len(),
            );
            for c in char_map[self.loc.pos.line][self.loc.pos.col..highlighted_right_bound].iter() {
                output.push(*c);
            }
            output += "\x1b[24m\x1b[2m";
            for c in char_map[self.loc.pos.line][highlighted_right_bound..right_bound].iter() {
                output.push(*c);
            }
        }
        // printing line below
        output += try_get_ln(1).as_str();
        output += try_get_ln(2).as_str();

        output += "\x1b[22m\n";
        output += self.msg.as_str();
        return output;
    }
}

/// Types of handled errors
#[derive(Debug)]
pub enum ErrorType {
    SyntaxError,
    ConveyorBeltError,
    IdentifierError,
    RuntimeError,
}
impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::SyntaxError => "Syntax Error",
            Self::ConveyorBeltError => "Conveyor Belt Error",
            Self::IdentifierError => "Identifier Error",
            Self::RuntimeError => "Runtime Error",
        };
        write!(f, "{s}")
    }
}
