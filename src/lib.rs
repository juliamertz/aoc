mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use std::fmt::{Debug, Display};

pub use anyhow::Result;
pub use itertools::Itertools;

pub fn regex(pattern: impl AsRef<str>) -> regex::Regex {
    regex::Regex::new(pattern.as_ref()).unwrap()
}

pub fn num(pattern: &str) -> u32 {
    pattern.parse().unwrap()
}

pub type Pos = (usize, usize);

#[derive(Debug, Clone)]
pub struct Grid<T> {
    lines: Vec<Vec<T>>,
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line.iter().map(|t| t.to_string()).join(" "))?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    fn new(cells: Vec<Vec<T>>) -> Grid<T> {
        Grid { lines: cells }
    }

    fn set(&mut self, (x, y): Pos, val: T) -> Result<()> {
        if self.get((x, y)).is_none() {
            anyhow::bail!("No such tile")
        }
        self.lines[y][x] = val;

        Ok(())
    }

    fn get(&self, pos: Pos) -> Option<&T> {
        let (x, y) = pos;
        self.lines.get(y).and_then(|l| l.get(x))
    }
}
