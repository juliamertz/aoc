use super::*;

pub fn solve(input: Input) -> u64 {
    input
        .into_iter()
        .map(|range| {
            let mut buf = 0;
            dbg!(&range);
            dbg!(range.start.is_valid());
            dbg!(range.end.is_valid());
            if !range.start.is_valid() {
                buf += range.start.0.parse::<u64>().unwrap();
            }
            if !range.end.is_valid() {
                buf += range.end.0.parse::<u64>().unwrap();
            }
            buf
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}
