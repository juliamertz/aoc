pub use super::*;

pub mod a;
pub mod b;

use std::str::FromStr;

use anyhow::anyhow;

pub type Removed = bool;

#[derive(Debug, PartialEq, Eq, Default)]
pub enum Tile {
    #[default]
    Empty,
    Roll(Removed),
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty | Self::Roll(true))
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => ".",
            Self::Roll(false) => "@",
            Self::Roll(true) => "x",
        })
    }
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "." => Self::Empty,
            "@" => Self::Roll(Default::default()),
            _ => return Err(anyhow!("invalid tile: '{s}'")),
        })
    }
}

pub type Input = Grid<Tile>;

pub fn parse_input(input: &str) -> Input {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|v| !v.is_empty())
                    .map(|tile| Tile::from_str(tile).unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    )
}
