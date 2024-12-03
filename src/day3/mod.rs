pub mod a;
pub mod b;

pub use super::*;
pub type Input = &'static str;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = include_str!("./input.txt");
        assert_eq!(a::solve(input).unwrap(), 175700056);
    }

    #[test]
    fn part_b() {
        let input = include_str!("./input.txt");
        assert_eq!(b::solve(input), 71668682);
    }
}
