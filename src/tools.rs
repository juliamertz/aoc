use colored::Colorize;
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    sync::{Arc, Mutex},
};

pub use anyhow::Result;
pub use itertools::Itertools;
pub use std::fmt::Display;
pub use strum_macros::EnumIter;

use crate::NUM_THREADS;

pub fn regex(pattern: impl AsRef<str>) -> regex::Regex {
    regex::Regex::new(pattern.as_ref()).unwrap()
}

pub fn num(pattern: &str) -> u32 {
    pattern.parse().unwrap()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Left, Self::Right, Self::Up, Self::Down];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// get absolute difference for x and y co-ordinate respectively
    pub fn abs_diff(&self, other: Self) -> (usize, usize) {
        (self.x.abs_diff(other.x), self.y.abs_diff(other.y))
    }

    pub fn to(&self, dir: Direction) -> Option<Self> {
        Some(
            match dir {
                Direction::Up => (self.x, self.y.checked_sub(1)?),
                Direction::Down => (self.x, self.y + 1),
                Direction::Left => (self.x.checked_sub(1)?, self.y),
                Direction::Right => (self.x + 1, self.y),
            }
            .into(),
        )
    }
}

impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Pos { x, y }
    }
}

impl From<Pos> for (usize, usize) {
    fn from(pos: Pos) -> (usize, usize) {
        (pos.x, pos.y)
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct Vertex(pub Pos, pub Pos);

impl Vertex {
    pub fn new(a: Pos, b: Pos) -> Self {
        Self(a, b)
    }

    pub fn contains(&self, val: Pos) -> bool {
        self.0 == val || self.1 == val
    }
}

impl From<(Pos, Pos)> for Vertex {
    fn from((a, b): (Pos, Pos)) -> Self {
        Vertex(a, b)
    }
}

impl From<Vertex> for (Pos, Pos) {
    fn from(v: Vertex) -> (Pos, Pos) {
        (v.0, v.1)
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.contains(other.0) && self.contains(other.1)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub lines: Vec<Vec<T>>,
}

impl<T: Display> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Grid::new(value)
    }
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

impl<T: Display> Grid<T> {
    pub fn new(cells: Vec<Vec<T>>) -> Grid<T> {
        Grid { lines: cells }
    }

    pub fn width(&self) -> usize {
        self.lines[0].len()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn positions_iter(&self) -> Vec<Pos> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(|x| Pos { x, y }).collect_vec())
            .collect_vec()
    }

    pub fn print(&self) {
        for line in self.lines.iter() {
            for cell in line.iter() {
                print!("{} ", cell)
            }
            println!()
        }
        println!()
    }

    pub fn print_colored(&self, colors: &HashMap<Pos, impl ToString>) {
        for (y, line) in self.lines.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                let curr_pos: Pos = (x, y).into();
                if let Some(color) = colors.get(&curr_pos) {
                    print!("{} ", cell.to_string().color(color.to_string()))
                } else {
                    print!("{} ", cell)
                }
            }
            println!()
        }
        println!()
    }

    pub fn set(&mut self, pos: Pos, val: T) -> Result<()> {
        if self.get(&pos).is_none() {
            anyhow::bail!("No such tile")
        }
        self.lines[pos.y][pos.x] = val;

        Ok(())
    }

    pub fn get(&self, pos: &Pos) -> Option<&T> {
        self.lines.get(pos.y).and_then(|l| l.get(pos.x))
    }
}

pub fn generate_combinations<T: Clone>(n: usize, values: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = vec![0; n];

    loop {
        result.push(current.iter().map(|&i| values[i].clone()).collect());

        let mut i = n;
        while i > 0 && current[i - 1] + 1 == values.len() {
            i -= 1;
        }

        if i == 0 {
            break;
        }

        current[i - 1] += 1;

        #[allow(clippy::needless_range_loop)]
        for j in i..n {
            current[j] = 0;
        }
    }

    result
}

pub fn generate_unique_combinations<T: Clone + Eq + Hash>(n: usize, values: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = vec![0; n];

    loop {
        let comb = current.iter().map(|&i| values[i].clone()).collect_vec();
        if comb.iter().all_unique() {
            result.push(comb);
        }

        let mut i = n;
        while i > 0 && current[i - 1] + 1 == values.len() {
            i -= 1;
        }

        if i == 0 {
            break;
        }

        current[i - 1] += 1;

        #[allow(clippy::needless_range_loop)]
        for j in i..n {
            current[j] = 0;
        }
    }

    result
}

pub fn parallel_accumulate<T, U, F>(data: Vec<T>, shared_accumulator: U, task: F) -> U
where
    T: Send + 'static + Clone,
    U: Send + 'static,
    F: Fn(Vec<T>, Arc<Mutex<U>>) + Send + Sync + 'static,
{
    // Split the data into chunks based on the number of threads
    let chunks = data
        .chunks(data.len().div_ceil(*NUM_THREADS.get().unwrap()))
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    // Wrap the accumulator in Arc<Mutex>
    let shared = Arc::new(Mutex::new(shared_accumulator));
    let task = Arc::new(task);

    // Spawn threads
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            let shared = Arc::clone(&shared);
            let task = Arc::clone(&task);
            std::thread::spawn(move || {
                task(chunk, shared);
            })
        })
        .collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Return the accumulated value
    Arc::try_unwrap(shared)
        .unwrap_or_else(|_| panic!("Failed to unwrap Arc"))
        .into_inner()
        .unwrap_or_else(|_| panic!("Failed to unwrap Mutex"))
}
