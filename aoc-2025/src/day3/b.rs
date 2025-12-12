use std::collections::HashSet;

use super::*;

fn calc_power(batteries: &[u32]) -> u32 {
    let mut buf = 0u32;
    for (idx, value) in batteries.iter().rev().enumerate() {
        buf += value * 10_u32.pow(idx.try_into().unwrap());
    }
    buf
}

fn gen_permutations(input: Vec<u32>) -> HashSet<Vec<u32>> {
    let mut result = HashSet::new();

    fn aux(pos: usize, mut buf: Vec<u32>, result: &mut HashSet<Vec<u32>>) {
        if pos == buf.len() {
            result.insert(buf);
            return;
        }

        for idx in pos..buf.len() {
            buf.swap(pos, idx);
            aux(idx + 1, buf.clone(), result);
            buf.swap(pos, idx);
        }
    }

    aux(0, input, &mut result);

    result
}

pub fn solve(input: Input) -> u64 {
    let mut sum: u64 = 0;
    for bank in input {
        // let largest_joltage = gen_permutations(&bank)
        //     .into_iter()
        //     .sorted_by(|a, b| (calc_power(a)).cmp(&calc_power(b)))
        //     .collect_vec()
        //     .pop()
        //     .as_ref()
        //     .map(calc_power)
        //     .expect("no silly mistakes");
        // sum += largest_joltage as u64;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day3::b::{calc_power, gen_permutations};

    // #[test]
    // fn test_permutations() {
    //     let batteries = vec![1, 2, 3];
    //     let permutations = gen_permutations(batteries);
    //     dbg!(permutations);
    //     assert!(false)
    // }

    #[test]
    fn test_power_calculation() {
        assert_eq!(calc_power(&[2, 4, 5, 1, 4 , 3]), 245143);
    }
}
