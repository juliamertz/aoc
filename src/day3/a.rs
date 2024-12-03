use lazy_static::lazy_static;
use regex::Regex;

use super::*;

lazy_static! {
    static ref mul_re: Regex = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();
}

pub fn solve(input: Input) -> Result<u32> {
    let mut res = 0;

    for equasion in mul_re.captures_iter(input) {
        let (_, [x, y]) = equasion.extract();
        let (x, y): (u32, u32) = (x.parse()?, y.parse()?);
        res += x * y;
        dbg!(res);
    }

    Ok(res)
}
