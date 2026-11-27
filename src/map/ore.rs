/// Ore types. A level contains at most four, and only those its target needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ore {
    Iron,
    Copper,
    Coal,
    Stone,
}
