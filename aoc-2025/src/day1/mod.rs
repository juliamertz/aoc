pub mod a;
pub mod b;

use core::panic;

pub use super::*;

pub enum Rotation {
    Left(u64),
    Right(u64),
}

pub type Input = Vec<Rotation>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let amount = chars.collect::<String>().parse().unwrap();
            match direction {
                'L' => Rotation::Left(amount),
                'R' => Rotation::Right(amount),
                _ => panic!("bad input: {line}"),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(a::solve(input), 1055);
    }

    #[test]
    fn part_b() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(b::solve(input), 6386);
    }
}
