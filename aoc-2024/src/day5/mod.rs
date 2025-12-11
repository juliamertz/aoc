pub mod a;
pub mod b;

pub use super::*;

pub type Rule = (u32, u32);
pub type OrderingRules = Vec<Rule>;
pub type Update = Vec<u32>;
pub type Input = (OrderingRules, Vec<Update>);

pub fn parse_input(input: &str) -> Input {
    let split = input.split("\n\n").collect_vec();

    let rules = split
        .first()
        .unwrap()
        .split("\n")
        .map(|v| v.split("|").map(num).collect_tuple::<Rule>().unwrap())
        .collect_vec();

    let updates = split
        .last()
        .unwrap()
        .split("\n")
        .filter(|v| !v.is_empty())
        .map(|v| v.split(",").map(num).collect_vec())
        .collect_vec();

    (rules, updates)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(a::solve(input), 5087);
    }

    #[test]
    fn part_b() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(b::solve(input), 4971);
    }
}
