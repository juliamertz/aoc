use super::*;
use a::*;

pub fn solve(input: Input) -> u64 {
    let operators = &[Operator::Add, Operator::Multiply, Operator::Concat];

    parallel_accumulate(input, 0, move |data, acc| {
        for eq in data {
            if equasion_is_valid(&eq, operators) {
                *acc.lock().unwrap() += eq.0;
            }
        }
    })
}
