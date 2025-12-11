pub mod a;
pub mod b;

use std::str::FromStr;

pub use super::*;

pub type Board = Grid<Tile>;
pub type Placed = bool;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    /// Placed indicates if it has been manually placed to obstruct the gaurds path
    Obstacle(Placed),
    Guard(Orientation),
    Empty,
    Visited,

    // part b pretty printing purposes
    VisitedFacing(Orientation),
    Corner,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => "^",
                Self::Down => "v",
                Self::Right => ">",
                Self::Left => "<",
            }
        )
    }
}

impl Orientation {
    pub fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
            Self::Left => Self::Up,
        }
    }

    pub fn move_pos(&self, pos: Pos) -> Option<Pos> {
        let (x, y) = pos.into();
        Some(
            match self {
                Self::Up => (x, y.checked_sub(1)?),
                Self::Down => (x, y + 1),
                Self::Left => (x.checked_sub(1)?, y),
                Self::Right => (x + 1, y),
            }
            .into(),
        )
    }
}

impl FromStr for Orientation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "^" => Self::Up,
            "v" => Self::Down,
            ">" => Self::Right,
            "<" => Self::Left,
            _ => anyhow::bail!("invalid orientation: {s}"),
        })
    }
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "." => Tile::Empty,
            "#" => Tile::Obstacle(false),
            _ => Tile::Guard(Orientation::from_str(s)?),
        })
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ".".to_string(),
                Tile::Obstacle(placed) => match placed {
                    false => "#".to_string(),
                    true => "O".to_string(),
                },
                Tile::Guard(ref or) => or.to_string(),
                Tile::Visited => "X".to_string(),
                Tile::Corner => "+".to_string(),
                Tile::VisitedFacing(o) => match o {
                    Orientation::Left | Orientation::Right => "-".to_string(),
                    Orientation::Up | Orientation::Down => "|".to_string(),
                },
            }
        )
    }
}

pub fn parse_input(input: &str) -> Board {
    Grid::new(
        input
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.split("")
                    .filter_map(|ch| Tile::from_str(ch).ok())
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(a::solve(input), 4977);
    }

    #[test]
    fn part_b() {
        let input = parse_input(include_str!("./input.test.txt"));
        assert_eq!(b::solve(input), 6);
    }
}
