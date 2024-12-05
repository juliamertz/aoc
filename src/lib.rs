mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub use anyhow::Result;
pub use itertools::Itertools;

pub fn regex(pattern: impl AsRef<str>) -> regex::Regex {
    regex::Regex::new(pattern.as_ref()).unwrap()
}

pub fn num(pattern: &str) -> u32 {
    pattern.parse().unwrap()
}
