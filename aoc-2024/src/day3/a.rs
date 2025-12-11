use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref mul_re: Regex = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();
}

pub fn solve(input: &str) -> u32 {
    let mut res = 0;

    for equasion in mul_re.captures_iter(input) {
        let (_, [x, y]) = equasion.extract();
        let (x, y): (u32, u32) = (x.parse().unwrap(), y.parse().unwrap());
        res += x * y;
        dbg!(res);
    }

    res
}
