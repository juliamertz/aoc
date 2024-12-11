pub mod a;
pub mod b;

pub use super::*;
pub type Map = Grid<usize>;

pub fn parse_input(input: &str) -> Map {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|v| v.to_string().parse().unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    )
}
