use super::*;

pub fn solve(input: Input) -> u32 {
    let a = input.0.into_iter().sorted();
    let b = input.1.into_iter().sorted().collect_vec();

    a.enumerate().fold(0, |acc, (i, x)| acc + x.abs_diff(b[i]))
}
