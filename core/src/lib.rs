#[cfg(test)]
mod tests;

/// All valid belt characters
pub const BELTS: &str = "─│┌┐└┘═║╔╗╚╝";

/// All single belt characters
pub const SINGLE_BELTS: &str = "─│┌┐└┘";

/// All double belt characters
pub const DOUBLE_BELTS: &str = "═║╔╗╚╝";

/// All north-connecting belt characters
pub const NORTH_BELTS: &str = "│└┘║╚╝";

/// All east-connecting belt characters
pub const EAST_BELTS: &str = "─┌└═╔╚";

/// All south-connecting belt characters
pub const SOUTH_BELTS: &str = "│┌┐║╔╗";

/// All west-connecting belt characters
pub const WEST_BELTS: &str = "─┐┘═╗╝";

/// Defines a station and all the required information and functionality
#[derive(Debug)]
pub struct StationType<'a> {
    /// Identifier
    pub id: &'a str,
    /// Alternate identifier
    pub alt_id: Option<&'a str>,
    /// Minimum number of inputs required for this station to trigger its procedure
    pub inputs: usize,
    /// Does this station produce an output pallet
    pub output: bool,
    /// Station's procedure, takes a vector of input pallets and returns an optional
    /// pallet if successful, and an error message in a String if not
    pub procedure: fn(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String>,
}
impl StationType<'_> {
    /// Function to check whether a station has a certain ID
    pub fn has_id(&self, query: &str) -> bool {
        return self.id == query || (self.alt_id.is_some_and(|alt_id| alt_id == query));
    }
}

/// Instance of a pallet
#[derive(Debug, Clone, PartialEq)]
pub enum Pallet {
    Empty,
    Bool(bool),
    Char(char),
    String(String),
    Int(i32),
    Float(f32),
}
impl std::fmt::Display for Pallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pallet::Empty => String::from("Pallet::Empty"),
                Pallet::Bool(b) => format!("Pallet::Bool({})", if *b { "true" } else { "else" }),
                Pallet::Char(c) => format!("Pallet::Char(\'{}\')", c),
                Pallet::String(s) => format!("Pallet::String(\"{}\")", s),
                Pallet::Int(i) => format!("Pallet::Int({})", i),
                Pallet::Float(f) => format!("Pallet::Float({})", f),
            },
        )
    }
}

/* BROKEN
want to fix but I can't figure out the procedure definition macro stuff
because of macro hygeine

/// Macro to define a station in a library.
#[macro_export]
macro_rules! define_station {
    (
        $name:ident {
            id: $id:literal,
            inputs: $inputs:expr,
            output: $output:expr,
            procedure: $procedure:item
        }
    ) => {
        paste! {

            $procedure
            #[allow(non_upper_case_globals)]
            static [<$name>]: StationType = StationType {
                id: $id,
                inputs: $inputs,
                output: $output,
                procedure: [<$name:snake _procedure>],
            };
        }
    };
}
*/
