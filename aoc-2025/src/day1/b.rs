use super::*;
use super::day1::a::Dial;

pub fn solve(input: Input) -> u64 {
    let dial = Dial::default().apply_rotations(input);
    dial.full_count
}
