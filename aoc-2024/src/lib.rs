pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;

pub use aoc_common::*;

pub fn solve_day(day: u8, part: Part, test: bool) -> anyhow::Result<String> {
    let day_name = format!("day{}", day);
    let path = if test {
        format!("aoc-2024/src/{}/input.test.txt", day_name)
    } else {
        format!("aoc-2024/src/{}/input.txt", day_name)
    };
    let content = std::fs::read_to_string(path)?;
    
    let ans = match day {
        1 => solve_part!(part, day1, content),
        2 => solve_part!(part, day2, content),
        3 => solve_part!(part, day3, content),
        4 => solve_part!(part, day4, content),
        5 => solve_part!(part, day5, content),
        6 => solve_part!(part, day6, content),
        7 => solve_part!(part, day7, content),
        8 => solve_part!(part, day8, content),
        9 => solve_part!(part, day9, content),
        10 => solve_part!(part, day10, content),
        11 => solve_part!(part, day11, content),
        12 => solve_part!(part, day12, content),
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

