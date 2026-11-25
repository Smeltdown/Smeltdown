use crate::buildings::Building;
use crate::map::Map;

/// Generator output. The player phase only receives `map`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level {
    pub map: Map,
    /// Buildings of the reference solution, placed on `map`.
    pub reference: Vec<Building>,
}