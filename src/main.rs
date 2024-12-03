mod day1;
mod day2;
mod day3;
mod day4;

pub use itertools::Itertools;
pub use anyhow::Result;

fn main() {
    let input = include_str!("./day4/input.test.txt");
    day4::a::solve(input);
}
