use strum::IntoEnumIterator;

use super::*;

pub fn make_operator_comb(n: usize) -> Vec<Operator> {
    let mut buf: Vec<Operator> = vec![];

    let mut i = 0;
    while i < n {
        let op = Operator::iter().nth(i);
        i += 1;
    }

    buf
}

pub fn equasion_is_valid(eq: &Equasion) -> bool {
    let (answer, values) = eq;

    // let operator_amount = values.len() - 1;
    // let operator_combinations = {
    //
    //     for i in 0..operator_amount {
    //         let op =
    //     }
    //
    //     buf
    // };

    // let curr_value = values.first().unwrap();
    // let next_value = values.get(1).unwrap();
    //
    // for op in Operator::iter() {
    //     let calculated = op.apply(curr_value, next_value);
    //     dbg!(calculated, answer, calculated == *answer);
    //     if op.apply(curr_value, next_value) == *answer {
    //         return true
    //     }
    // }

    false
}

pub fn solve(input: Input) -> u32 {
    let mut ans = 0;

    for eq in input {
        if equasion_is_valid(&eq) {
            ans += eq.0;
        };
    }

    dbg!(ans)
}
