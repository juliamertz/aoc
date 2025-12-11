pub mod a;
pub mod b;

pub use super::*;

#[derive(Debug)]
pub struct Id<'a>(pub &'a str);

impl<'a> From<&'a str> for Id<'a> {
    fn from(value: &'a str) -> Self {
        Self(value.trim())
    }
}

#[derive(Debug)]
pub struct Range<Idx = u64> {
    start: Idx,
    end: Idx,
}

pub type Input = Vec<Range>;

pub fn parse_input(input: &str) -> Input {
    input
        .split(",")
        .map(|item| {
            item.split_once("-")
                .map(|(start, end)| Range {
                    start: start.trim().parse().unwrap(),
                    end: end.trim().parse().unwrap(),
                })
                .unwrap()
        })
        .collect()
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn part_a() {
//         let input = parse_input(include_str!("./input.txt"));
//         assert_eq!(a::solve(input), 1055);
//     }
//
//     #[test]
//     fn part_b() {
//         let input = parse_input(include_str!("./input.txt"));
//         assert_eq!(b::solve(input), 6386);
//     }
// }
