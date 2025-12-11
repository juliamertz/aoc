pub mod day1;
pub mod day2;

pub use aoc_common::*;

pub fn solve_day(day: u8, part: Part, test: bool) -> anyhow::Result<String> {
    let day_name = format!("day{}", day);
    let path = if test {
        format!("aoc-2025/src/{}/input.test.txt", day_name)
    } else {
        format!("aoc-2025/src/{}/input.txt", day_name)
    };
    let content = std::fs::read_to_string(path)?;
    
    let ans = match day {
        1 => solve_part!(part, day1, content),
        2 => solve_part!(part, day2, content),
        _ => anyhow::bail!("Day {} not implemented", day),
    };
    
    Ok(ans)
}

#[macro_export]
macro_rules! solve_part {
    ($part:expr, $day:ident, $content:expr) => {
        match $part {
            Part::A => $day::a::solve($day::parse_input(&$content)).to_string(),
            Part::B => $day::b::solve($day::parse_input(&$content)).to_string(),
        }
    };
}

#[derive(strum_macros::Display, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Part {
    A,
    B,
}

