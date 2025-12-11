use super::*;

fn calc_power(batteries: &[u8; 2]) -> u8 {
    batteries[0] * 10 + batteries[1]
}

fn gen_permutations(bank: &Bank) -> Vec<[u8; 2]> {
    let mut buf = vec![];
    let batteries = &bank.batteries;

    for (idx, battery) in batteries.iter().enumerate() {
        for offset in idx + 1..batteries.len() {
            buf.push([*battery, batteries[offset]]);
        }
    }

    buf
}

pub fn solve(input: Input) -> u64 {
    let mut sum: u64 = 0;
    for bank in input {
        let largest_joltage = gen_permutations(&bank)
            .into_iter()
            .sorted_by(|a, b| (calc_power(a)).cmp(&calc_power(b)))
            .collect_vec()
            .pop()
            .as_ref()
            .map(calc_power)
            .expect("no silly mistakes");
        sum += largest_joltage as u64;
    }

    sum
}
