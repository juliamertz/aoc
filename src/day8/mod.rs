pub mod a;
pub mod b;

use std::fmt::Display;

pub use super::*;

pub type Input = Grid<Tile>;
pub type Frequency = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Antenna(Frequency),
    Antinode,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ".".into(),
                Tile::Antinode => "#".into(),
                Tile::Antenna(freq) => freq.to_string(),
            }
        )
    }
}

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => Tile::Antinode,
                    '.' => Tile::Empty,
                    freq => Tile::Antenna(freq),
                })
                .collect_vec()
        })
        .collect_vec()
        .into()
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn part_a() {
//         let input = parse_input(include_str!("./input.txt"));
//         assert_eq!(a::solve(input), 1579939);
//     }
//
//     #[test]
//     fn part_b() {
//         let input = parse_input(include_str!("./input.txt"));
//         assert_eq!(b::solve(input), 20351745);
//     }
// }
