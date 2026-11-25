mod hardcoded_level;

use crate::map::Map;
use crate::map::difficulty::Difficulty;
use crate::map::level::Level;
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
