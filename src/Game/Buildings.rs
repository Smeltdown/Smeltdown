
#[derive (Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive (Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),  
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}


#[derive (Debug, Clone, Copy, PartialEq, Eq ,Hash)]
pub enum Items {
    IronOre,
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
    direction: Direction,
    produces: Items,
    production_interval: u32,
}

#[derive (Debug, Clone)]
pub struct BeltConfig {
    position: Position,
    direction: Direction,
    capacity: usize,
    transit_ticks: u32,
}

#[derive (Debug, Clone)]
pub struct DeliveryPointConfig {
    position: Position,
    direction: Direction,
    target_item: Vec<Items>,
    target_per_minute: u32,
}

#[derive (Debug, Clone)]
pub struct Layout {
    pub width: u32,
    pub height: u32,
    pub miners: Vec<MinerConfig>,
    pub belts: Vec<BeltConfig>,
    pub delivery_point: Vec<DeliveryPointConfig>
}

pub enum Target {
    Belt(usize),
    DeliveryPoint(usize),
    None
}

impl Layout {
    pub fn belt_at(&self, position: Position) -> Option<usize> {
        self.belts.iter().position(|belt| belt.position == position)
    }
    pub fn delivery_point_at(&self, position: Position) -> Option<usize> {
        self.delivery_point.iter().position(|dp| dp.position == position)
    }
    pub fn next_to_belt(&self, belt_idx: usize) -> Target {
        let belt = &self.belts[belt_idx];
        let next_position = match belt.direction {
            Direction::North => Position { x: belt.position.x, y: belt.position.y - 1 },
            Direction::East => Position { x: belt.position.x + 1, y: belt.position.y },
            Direction::South => Position { x: belt.position.x, y: belt.position.y + 1 },
            Direction::West => Position { x: belt.position.x - 1, y: belt.position.y },
        };
        if let Some(next_belt_idx) = self.belt_at(next_position) {
            Target::Belt(next_belt_idx)
        } else if let Some(next_dp_idx) = self.delivery_point_at(next_position) {
            Target::DeliveryPoint(next_dp_idx)
        } else {
            Target::None
        }
    }
    pub fn output_of_miner(&self, miner_idx: usize) -> Target {
        let miner = &self.miners[miner_idx];
        let next_position = match miner.direction {
            Direction::North => Position { x: miner.position.x, y: miner.position.y - 1 },
            Direction::East => Position { x: miner.position.x + 1, y: miner.position.y },
            Direction::South => Position { x: miner.position.x, y: miner.position.y + 1 },
            Direction::West => Position { x: miner.position.x - 1, y: miner.position.y },
        };
        if let Some(next_belt_idx) = self.belt_at(next_position) {
            Target::Belt(next_belt_idx)
        } else {
            Target::None
        }
    }    
}


