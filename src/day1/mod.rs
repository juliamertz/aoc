pub mod a;
pub mod b;

pub use itertools::Itertools;

pub type Input = (Vec<u32>, Vec<u32>);
fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let split = l.split_whitespace().collect_vec();
            (
                split[0].parse::<u32>().unwrap(),
                split[1].parse::<u32>().unwrap(),
            )
        })
        .unzip()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(a::solve(input), 1579939);
    }

    #[test]
    fn part_b() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(b::solve(input), 20351745);
    }
}
