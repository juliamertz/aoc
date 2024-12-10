pub mod a;
pub mod b;

pub use super::*;

pub struct Input {
    content: Vec<Vec<char>>,
}

impl Input {
    pub fn get(&self, pos: Pos) -> Option<&char> {
        let (x, y) = pos.into();
        self.content.get(y).and_then(|line| line.get(x))
    }
}

pub fn parse_input(input: &str) -> Input {
    Input {
        content: input.lines().map(|l| l.chars().collect_vec()).collect_vec(),
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn part_a() {
//         let input = parse_input(include_str!("./input.txt"));
//         assert_eq!(a::solve(input), ());
//     }
//
//     #[test]
//     fn part_b() {
//         let input = parse_input(include_str!("./input.txt"));
//         assert_eq!(b::solve(input), ());
//     }
// }
