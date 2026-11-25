use crate::buildings::Building;
use crate::map::Map;

/// A map plus what the player built on it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blueprint {
    /// Read-only generated Map
    map: Map,
    pub buildings: Vec<Building>,
}

impl Blueprint {
    pub fn new(map: Map) -> Self {
        Self {
            map,
            buildings: Vec::new(),
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }
}
