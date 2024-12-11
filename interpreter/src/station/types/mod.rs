use std::fmt;

pub mod builtin;

use builtin::BuiltinStationType;

#[derive(Debug)]
pub enum StationType {
    /// Builtin station, with type data
    Builtin(&'static BuiltinStationType),
    /// Function invocation, with function id
    FuncInvoke(usize),
    /// Function input, with function id and arg index
    FuncInput((usize, usize)),
    /// Function output, with function id
    FuncOutput(usize),
}
impl StationType {}
impl fmt::Display for StationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        return Ok(match &self {
            Self::Builtin(s) => write!(f, "{}", s.id),
            Self::FuncInvoke(id) => write!(f, "f{}.invoke", id),
            Self::FuncInput((id, arg_i)) => write!(f, "f{}.input.{}", id, arg_i),
            Self::FuncOutput(id) => write!(f, "f{}.output", id),
        }?);
    }
}
