use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    sync::{Arc, Mutex},
};

pub use anyhow::Result;
use colored::Colorize;
pub use itertools::Itertools;
pub use strum_macros::EnumIter;

use crate::NUM_THREADS;

pub fn regex(pattern: impl AsRef<str>) -> regex::Regex {
    regex::Regex::new(pattern.as_ref()).unwrap()
}

pub fn num(pattern: &str) -> u32 {
    pattern.parse().unwrap()
}

// pub type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    /// get absolute difference for x and y co-ordinate respectively
    pub fn abs_diff(&self, other: Self) -> (usize, usize) {
        (self.x.abs_diff(other.x), self.y.abs_diff(other.y))
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

#[derive(Debug)]
pub struct Vertex(Pos, Pos);

impl Vertex {
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

    pub fn print_colored(&self, colors: HashMap<Pos, impl ToString>) {
        for (y, line) in self.lines.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if let Some(color) = colors.get(&(x, y).into()) {
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
        let (x, y) = pos.into();

        if self.get((x, y).into()).is_none() {
            anyhow::bail!("No such tile")
        }
        self.lines[y][x] = val;

        Ok(())
    }

    pub fn get(&self, pos: Pos) -> Option<&T> {
        let (x, y) = pos.into();
        self.lines.get(y).and_then(|l| l.get(x))
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
        .chunks(data.len().div_ceil(NUM_THREADS))
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
