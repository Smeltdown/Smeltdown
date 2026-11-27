mod hardcoded_level;

use crate::map::Map;
use crate::map::TileKind;
use crate::map::difficulty::Difficulty;
use crate::map::grid::Position;
use crate::map::level::Level;
use crate::map::ore::Ore;
use hardcoded_level::hardcoded_level;

/// Stub: only records `seed` and `difficulty`, always returns the same 8 × 4 map
/// and an empty reference solution.
pub fn generate_level(seed: &str, difficulty: Difficulty) -> Level {
    Level {
        map: Map::new(seed.to_string(), difficulty, 8, 4, hardcoded_level())
            .expect("hardcoded map is valid"),
        reference: vec![],
    }
}

/// Prints an entire level into cli
pub fn print_level(level: &Level) {
    let map = &level.map;
    println!(
        "seed: {}\ndifficulty: {:?}\nwidth / height: {} / {}\n",
        map.seed(),
        map.difficulty(),
        map.width(),
        map.height()
    );
    let border = "─".repeat(map.width() as usize * 2 + 1);
    println!("┌{border}┐");
    for y in 0..map.height() {
        let row = (0..map.width())
            .map(
                |x| match map.tiles().iter().find(|t| t.pos == Position { x, y }) {
                    None => ".",
                    Some(t) => match t.kind {
                        TileKind::Rock => "R",
                        TileKind::DeliveryPoint => "D",
                        TileKind::Ore(Ore::Iron) => "I",
                        TileKind::Ore(Ore::Copper) => "C",
                        TileKind::Ore(Ore::Coal) => "L",
                        TileKind::Ore(Ore::Stone) => "S",
                    },
                },
            )
            .collect::<Vec<_>>()
            .join(" ");
        println!("│ {row} │");
    }
    println!("└{border}┘");
    println!("\nLegend: I=iron;  Cu=copper;  L=coal;  S=stone;  R=rock;  D=delivery;  .=empty");
}
