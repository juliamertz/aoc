pub mod a;
pub mod b;

pub use super::*;
pub type Input = Vec<Block>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Block {
    Free(usize),
    File {
        id: u64,
        size: usize,
        start: usize,
        end: usize,
    },
}

pub fn parse_input(input: &str) -> Vec<Block> {
    let mut bits = 0;
    let mut id = 0;
    dbg!(&input);
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            let size = ch.to_string().parse().unwrap();
            let is_even = i % 2 == 0;
            let block = match is_even {
                true => Block::File {
                    id,
                    size,
                    start: bits,
                    end: bits + size,
                },
                false => Block::Free(size),
            };

            bits += size;

            if is_even {
                id += 1;
            }

            block
        })
        .collect_vec()
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn part_a() {
//         let input = parse_input(include_str!("./input.txt"));
//         assert_eq!(a::solve(input), 1579939);
//     }
//
//     #[test]
//     fn part_b() {
//         let input = parse_input(include_str!("./input.txt"));
//         assert_eq!(b::solve(input), 20351745);
//     }
// }
