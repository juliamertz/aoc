mod day1;
mod day2;
mod day3;
mod day4;

pub use anyhow::Result;
pub use itertools::Itertools;

fn main() {
    let input = day4::parse_input(include_str!("./day4/input.txt"));
    day4::b::solve(input);
}
