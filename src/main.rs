mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub mod tools;
use std::time::{Duration, Instant};

pub use tools::*;

/// To speed up any nasty brute forces...
const NUM_THREADS: usize = 24;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().skip(1).collect_vec();

    match args.iter().map(|f| f.as_str()).collect_vec().as_slice() {
        ["solve", day_number, part, test] => {
            let day_number: usize = day_number.parse()?;
            if day_number > 25 {
                anyhow::bail!("No such day {day_number}")
            }

            let test = *test == "true";
            let part = part.to_owned();
            match day_number {
                1 => solve!(day1, part, test),
                2 => solve!(day2, part, test),
                3 => solve!(day3, part, test),
                4 => solve!(day4, part, test),
                5 => solve!(day5, part, test),
                6 => solve!(day6, part, test),
                7 => solve!(day7, part, test),
                8 => solve!(day8, part, test),
                9 => solve!(day9, part, test),
                _ => unimplemented!(),
            };
        }
        _ => anyhow::bail!("what"),
    };

    Ok(())
}

#[macro_export]
macro_rules! solve_part {
    ($part:expr, $day:ident, $input:expr) => {
        match $part {
            "a" => $day::a::solve($input).to_string(),
            "b" => $day::b::solve($input).to_string(),
            _ => unimplemented!(),
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
