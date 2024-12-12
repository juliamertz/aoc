mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;

use argh::{FromArgValue, FromArgs};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use strum_macros::Display;

pub mod tools;
pub use tools::*;

#[derive(FromArgs)]
/// Festive pain 🥳😭
struct Cli {
    /// to speed up any nasty brute forces...
    #[argh(option)]
    threads: Option<usize>,

    /// whether or not to use test input
    #[argh(switch, short = 't')]
    test: bool,

    #[argh(positional)]
    day: u8,

    #[argh(positional)]
    part: Part,
}

pub static NUM_THREADS: OnceLock<usize> = OnceLock::new();

fn init_globals(args: &Cli) {
    _ = NUM_THREADS.set(args.threads.unwrap_or(24));
}

fn main() -> anyhow::Result<()> {
    let args: Cli = argh::from_env();
    init_globals(&args);

    let (part, test, day) = (args.part, args.test, args.day);
    if day > 25 {
        anyhow::bail!("No such day {}", args.day)
    }

    match day {
        1 => solve!(day1, part, test),
        2 => solve!(day2, part, test),
        3 => solve!(day3, part, test),
        4 => solve!(day4, part, test),
        5 => solve!(day5, part, test),
        6 => solve!(day6, part, test),
        7 => solve!(day7, part, test),
        8 => solve!(day8, part, test),
        9 => solve!(day9, part, test),
        10 => solve!(day10, part, test),
        11 => solve!(day11, part, test),
        12 => solve!(day12, part, test),
        

        _ => unimplemented!(),
    };

    Ok(())
}

#[macro_export]
macro_rules! solve_part {
    ($part:expr, $day:ident, $input:expr) => {
        match $part {
            Part::A => $day::a::solve($input).to_string(),
            Part::B => $day::b::solve($input).to_string(),
        }
    };
}

#[macro_export]
macro_rules! solve {
    ($day:ident,$part:tt,$test:expr) => {{
        let path = if $test {
            format!("src/{}/input.test.txt", stringify!($day))
        } else {
            format!("src/{}/input.txt", stringify!($day))
        };
        let content = std::fs::read_to_string(path).unwrap();
        let input = $day::parse_input(&content);

        let start = Instant::now();
        let ans = solve_part!($part, $day, input);
        let duration = start.elapsed();

        println!("Answer for {} part {}: {ans}", stringify!($day), $part);
        println!("Time taken {}", fmt_duration(duration));
    }};
}

fn fmt_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let micros = duration.subsec_micros();
    let nanos = duration.subsec_nanos();

    if secs > 0 {
        format!("{} s", secs)
    } else if millis > 0 {
        format!("{} ms", millis)
    } else if micros > 0 {
        format!("{} μs", micros)
    } else {
        format!("{} ns", nanos)
    }
}

#[derive(EnumIter, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Part {
    A,
    B,
}

impl FromArgValue for Part {
    fn from_arg_value(value: &str) -> std::result::Result<Self, String> {
        match value.to_string().to_lowercase().as_str() {
            "a" => Ok(Part::A),
            "b" => Ok(Part::B),
            _ => Err("Choose from either A or B".into()),
        }
    }
}
