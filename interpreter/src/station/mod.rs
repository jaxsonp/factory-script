pub mod modifiers;
pub mod types;

pub use modifiers::StationModifiers;

use crate::{util::*, Pallet, *};
use types::{StationType, STATION_TYPES};

/// Instance of a station
#[derive(Debug)]
pub struct Station {
    /// Location of the station in source code
    pub loc: SourceSpan,
    /// Station type information
    pub s_type: &'static StationType,
    /// Data the station may need
    pub data: StationData,
    /// Modifiers
    pub modifiers: StationModifiers,
    /// Queues for each input bay
    pub in_bays: Vec<Option<Pallet>>,
    /// Map of each output bay connection in the form (station_index, in_bay_index)
    pub out_bays: Vec<(usize, usize)>,
}
impl Station {
    pub fn from_str(identifier: &str, loc: SourceSpan) -> Result<Self, Error> {
        for station_type in STATION_TYPES.iter() {
            if station_type.has_id(identifier) {
                return Ok(Self {
                    loc,
                    s_type: *station_type,
                    modifiers: StationModifiers::default(),
                    in_bays: Vec::new(),
                    out_bays: Vec::new(),
                    data: StationData::None,
                });
            }
        }
        return Err(Error::new(
            IdentifierError,
            loc,
            format!("Failed to find station type with identifier \"{identifier}\"").as_str(),
        ));
    }

    pub fn clear_in_bays(&mut self) {
        for bay in self.in_bays.iter_mut() {
            if bay.is_some() {
                *bay = None;
            }
        }
    }
}

#[derive(Debug)]
pub enum StationData {
    AssignValue(Pallet),
    FunctionID(usize),
    FunctionIDAndIndex(usize, usize),
    None,
}
