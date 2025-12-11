use argh::FromArgs;
use std::time::{Duration, Instant};

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
    year: u16,

    #[argh(positional)]
    day: u8,

    #[argh(positional)]
    part: Part,
}

#[derive(strum_macros::Display, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Part {
    A,
    B,
}

impl argh::FromArgValue for Part {
    fn from_arg_value(value: &str) -> std::result::Result<Self, String> {
        match value.to_string().to_lowercase().as_str() {
            "a" => Ok(Part::A),
            "b" => Ok(Part::B),
            _ => Err("Choose from either A or B".into()),
        }
    }
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

fn main() -> anyhow::Result<()> {
    let args: Cli = argh::from_env();
    
    // Initialize num_threads for parallel operations
    aoc_common::init_num_threads(args.threads.unwrap_or(24));
    
    if args.day > 25 {
        anyhow::bail!("No such day {}", args.day)
    }

    let start = Instant::now();
    let ans = match args.year {
        2024 => aoc_2024::solve_day(args.day, convert_part(args.part), args.test)?,
        2025 => aoc_2025::solve_day(args.day, convert_part_2025(args.part), args.test)?,
        _ => anyhow::bail!("Year {} not implemented", args.year),
    };
    let duration = start.elapsed();

    println!("Answer for {} day {} part {}: {}", args.year, args.day, args.part, ans);
    println!("Time taken: {}", fmt_duration(duration));

    Ok(())
}

fn convert_part(part: Part) -> aoc_2024::Part {
    match part {
        Part::A => aoc_2024::Part::A,
        Part::B => aoc_2024::Part::B,
    }
}

fn convert_part_2025(part: Part) -> aoc_2025::Part {
    match part {
        Part::A => aoc_2025::Part::A,
        Part::B => aoc_2025::Part::B,
    }
}

