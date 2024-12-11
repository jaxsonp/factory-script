pub mod constants;

/// Instance of a pallet
#[derive(Debug, Clone, PartialEq)]
pub enum Pallet {
    Empty,
    Bool(bool),
    Char(char),
    String(String),
    Int(i64),
    Float(f64),
}
impl std::fmt::Display for Pallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pallet::Empty => String::from("Pallet< >"),
                Pallet::Bool(b) => format!("Pallet<b:{}>", if *b { "true" } else { "false" }),
                Pallet::Char(c) => format!("Pallet<c:\'{}\'>", c),
                Pallet::String(s) => format!("Pallet<s:\"{}\">", s),
                Pallet::Int(i) => format!("Pallet<i:{}>", i),
                Pallet::Float(f) => format!("Pallet<f:{}>", f),
            },
        )
    }
}
