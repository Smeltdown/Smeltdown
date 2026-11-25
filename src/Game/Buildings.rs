#[derive (Debug Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive (Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive (Debug, Clone, Copy, PartialEq, Eq,Hash)]
pub enum Items {
    Iron_Ore,
}

/*
#[derive (Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Items {
pub id: u32
}
*/

#[derive (Debug, Clone)]
pub struct MinerConfig {
    position: Position,
    produces: Items,
    Production_interval: u32,
}

#[derive (Debug, Clone)]
pub struct BeltConfig {
    position: Position,
    direction: Direction,
    capacity: usize,
    transit_ticks: u32,
}

#[derive (Debug, Clone)]
pub struct Delivery_PointConfig {
    position: Position,
    Target_item: Vec<Items>,
    Target_per_minute: u32,
}
