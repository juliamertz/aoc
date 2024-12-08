use super::*;

pub fn solve(input: Input) -> u32 {
    let (a, b) = input;

    a.iter().fold(0, |acc, x| {
        let occurunces = b.iter().filter(|y| y == &x).collect_vec().len();

        acc + (x * occurunces as u32)
    })
}
