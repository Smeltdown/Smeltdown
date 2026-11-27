mod buildings;
mod generator;
mod map;

fn main() {
    let level = generator::generate_level("test", map::difficulty::Difficulty::Easy);
    generator::print_level(&level);
}
