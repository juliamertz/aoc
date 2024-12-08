/// this got really messy
use lazy_static::lazy_static;
use regex::Regex;

use super::*;

lazy_static! {
    static ref mul_re: Regex = Regex::new(r#"mul\(\d{1,3},\d{1,3}\)"#).unwrap();
    static ref do_dont_re: Regex = Regex::new(r#"(do\(\)|don't\(\))"#).unwrap();
}

pub fn solve(input: &str) -> u32 {
    let mut res = 0;
    let mut enabled = true;
    let captures = mul_re.find_iter(input).collect_vec();

    for (i, eq) in captures.iter().enumerate() {
        let content = eq.as_str();

        let (x, y) = content
            .strip_prefix("mul(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(",")
            .map(|v| v.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        if enabled {
            res += x * y;
        }

        if let Some(next) = captures.get(i + 1) {
            let (_, next_content) = input.split_at(eq.end());
            let (next_content, _) = next_content.split_at(next.start() - eq.end());

            for instruction in do_dont_re.find_iter(next_content) {
                let c = instruction.as_str();
                match c {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    _ => unreachable!(),
                }
            }
        }
    }

    res
}
