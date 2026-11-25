use crate::map::grid::Position;
use crate::map::ore::Ore;
use crate::map::{Tile, TileKind};

/// Tiles of the hardcoded 8 × 4 level.
pub fn hardcoded_level() -> Vec<Tile> {
    vec![
        Tile {
            kind: TileKind::Ore(Ore::Iron),
            pos: Position { x: 0, y: 0 },
        },
        Tile {
            kind: TileKind::Ore(Ore::Iron),
            pos: Position { x: 1, y: 0 },
        },
        Tile {
            kind: TileKind::Ore(Ore::Iron),
            pos: Position { x: 0, y: 1 },
        },
        Tile {
            kind: TileKind::Ore(Ore::Iron),
            pos: Position { x: 1, y: 1 },
        },
        Tile {
            kind: TileKind::Rock,
            pos: Position { x: 4, y: 0 },
        },
        Tile {
            kind: TileKind::Rock,
            pos: Position { x: 3, y: 1 },
        },
        Tile {
            kind: TileKind::Rock,
            pos: Position { x: 5, y: 2 },
        },
        Tile {
            kind: TileKind::Rock,
            pos: Position { x: 1, y: 3 },
        },
        Tile {
            kind: TileKind::DeliveryPoint,
            pos: Position { x: 6, y: 1 },
        },
        Tile {
            kind: TileKind::DeliveryPoint,
            pos: Position { x: 7, y: 1 },
        },
        Tile {
            kind: TileKind::DeliveryPoint,
            pos: Position { x: 6, y: 2 },
        },
        Tile {
            kind: TileKind::DeliveryPoint,
            pos: Position { x: 7, y: 2 },
        },
    ]
}