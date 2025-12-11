pub mod a;
pub mod b;

pub use super::*;

pub type Battery = u8;

pub struct Bank {
    batteries: Vec<Battery>,
}

pub type Input = Vec<Bank>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| Bank {
            batteries: line
                .chars()
                .map(|v| v.to_string().parse().unwrap())
                .collect_vec(),
        })
        .collect_vec()
}
