use strum::IntoEnumIterator;

use super::*;

fn generate_permutations(n: usize, operators: &[Operator]) -> Vec<Vec<Operator>> {
    let mut result = Vec::new();
    let mut current = vec![0; n];

    loop {
        result.push(current.iter().map(|&i| operators[i]).collect());

        let mut i = n;
        while i > 0 && current[i - 1] + 1 == operators.len() {
            i -= 1;
        }

        if i == 0 {
            break;
        }

        current[i - 1] += 1;

        for j in i..n {
            current[j] = 0;
        }
    }

    result
}

pub fn equasion_is_valid(eq: &Equasion, operators: &[Operator]) -> bool {
    let (answer, values) = eq;
    let permutations = generate_permutations(values.len() - 1, operators);

    for permutation in permutations.iter() {
        let mut buf = values.clone();
        let mut i = 0;

        while i < &permutations.len() - 1 {
            let curr = buf[i];

            if i + 1 == buf.len() {
                break;
            }

            let next = buf[i + 1];
            let res = permutation[i].apply(&curr, &next);
            if &res > answer {
                i += 1;
                continue;
            }
            buf[i + 1] = res;
            i += 1
        }

        if buf.last().unwrap() == answer {
            return true;
        }
    }

    false
}

pub fn solve(input: Input) -> u64 {
    let operators = &[Operator::Add, Operator::Multiply];
    let mut ans = 0;

    for eq in input {
        if equasion_is_valid(&eq, operators) {
            ans += eq.0;
        };
    }

    dbg!(ans)
}
