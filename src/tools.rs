use std::{
    fmt::{Debug, Display},
    sync::{Arc, Mutex},
};

pub use anyhow::Result;
pub use itertools::Itertools;
pub use strum_macros::EnumIter;

use crate::NUM_THREADS;

pub fn regex(pattern: impl AsRef<str>) -> regex::Regex {
    regex::Regex::new(pattern.as_ref()).unwrap()
}

pub fn num(pattern: &str) -> u32 {
    pattern.parse().unwrap()
}

pub type Pos = (usize, usize);

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub lines: Vec<Vec<T>>,
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
    pub fn new(cells: Vec<Vec<T>>) -> Grid<T> {
        Grid { lines: cells }
    }

    pub fn set(&mut self, (x, y): Pos, val: T) -> Result<()> {
        if self.get((x, y)).is_none() {
            anyhow::bail!("No such tile")
        }
        self.lines[y][x] = val;

        Ok(())
    }

    pub fn get(&self, pos: Pos) -> Option<&T> {
        let (x, y) = pos;
        self.lines.get(y).and_then(|l| l.get(x))
    }
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
