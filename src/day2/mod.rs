pub mod a;
pub mod b;

pub use super::*;
pub type Report = Vec<u32>;

pub fn parse_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|n| n.parse::<u32>().expect("valid integer"))
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(a::solve(input), 326);
    }

    #[test]
    fn part_b() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(b::solve(input), 381);
    }
}
