use super::*;

fn sum_invalid(range: Range) -> u64 {
    let mut buf = 0;
    if range.start < range.end {
        for idx in range.start..=range.end {
            let str = idx.to_string();
            if str.starts_with("0") {
                continue;
            }
            let (lhs, rhs) = str.split_at(str.len() / 2);
            if lhs == rhs {
                buf += idx;
            }
        }
    }
    buf
}

pub fn solve(input: Input) -> u64 {
    input
        .into_iter()
        .map(sum_invalid)
        .collect::<Vec<_>>()
        .iter()
        .sum()
}
