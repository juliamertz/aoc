pub use super::*;

pub mod a;
pub mod b;

pub type Garden = Grid<char>;

pub fn parse_input(input: &str) -> Garden {
    Grid::new(input.lines().map(|l| l.chars().collect_vec()).collect_vec())
}
