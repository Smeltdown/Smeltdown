pub mod blueprint;
pub mod difficulty;
pub mod grid;
pub mod level;
pub mod ore;

use difficulty::Difficulty;
use grid::Position;
use ore::Ore;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Rock,
    /// All delivery tiles together form the one delivery point.
    DeliveryPoint,
    Ore(Ore),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub kind: TileKind,
    pub pos: Position,
}

/// Generator output. Never changes during play. Positions without a tile are empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    seed: String,
    difficulty: Difficulty,
    width: u16,
    height: u16,
    tiles: Vec<Tile>,
}

impl Map {
    /// Fails if a tile is out of bounds or two tiles share a position.
    pub fn new(
        seed: String,
        difficulty: Difficulty,
        width: u16,
        height: u16,
        tiles: Vec<Tile>,
    ) -> Result<Self, String> {
        let mut seen = HashSet::new();
        for t in &tiles {
            if t.pos.x >= width || t.pos.y >= height {
                return Err(format!("{:?} is out of bounds", t.pos));
            }
            if !seen.insert(t.pos) {
                return Err(format!("{:?} is covered twice", t.pos));
            }
        }
        Ok(Self { seed, difficulty, width, height, tiles })
    }

    pub fn seed(&self) -> &str {
        &self.seed
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }
}