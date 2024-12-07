use strum::IntoEnumIterator;

use super::*;
use a::*;

pub fn solve(input: Input) -> u64 {
    let mut ans = 0;
    let operators = Operator::iter().collect_vec();

    for eq in input {
        let (answer, values) = eq.clone();

        let concatted: u64 = values
            .iter()
            .map(|v| v.to_string())
            .join("")
            .parse()
            .unwrap();
        if answer == concatted {
            ans += answer;
            continue;
        }

        if equasion_is_valid(&eq, &operators) {
            ans += eq.0;
        };
    }

    dbg!(ans)
}
