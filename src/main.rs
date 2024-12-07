mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub mod tools;
pub use tools::*;

const DAYS_AMOUNT: usize = 25;

fn main() -> anyhow::Result<()> {
    let input = include_str!("day7/input.txt");
    day7::Day7::a(day7::Day7::parse_input(input));

    std::process::exit(0);

    let args = std::env::args().collect_vec();

    match args.iter().map(|f| f.as_str()).collect_vec().as_slice() {
        ["solve", day_number] => {
            let day_number: usize = day_number.parse()?;
            if day_number > DAYS_AMOUNT {
                anyhow::bail!("No such day {day_number}")
            }

            // let solver: impl Solver = match day_number {
            //     1 => day1::Day1,
            //     2 => day2::Day2,
            //     3 => day3::Day3,
            //     4 => day4::Day4,
            //     5 => day5::Day5,
            //     6 => day6::Day6,
            //     _ => unimplemented!(),
            // };
        }
        _ => anyhow::bail!("what"),
    };

    dbg!(args);

    Ok(())
}

pub trait Solver {
    type Input;
    type Output;

    fn parse_input(data: &str) -> Self::Input;

    fn a(input: Self::Input) -> Self::Output;
    fn b(input: Self::Input) -> Self::Output;
}

#[macro_export]
macro_rules! solver_defaults {
    () => {
        fn a(input: Self::Input) -> Self::Output {
            a::solve(input)
        }

        fn b(input: Self::Input) -> Self::Output {
            b::solve(input)
        }
    };
}
