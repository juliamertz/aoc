pub mod a;
pub mod b;

pub use super::*;

pub type Input = Vec<Equasion>;
pub type Equasion = (u64, Vec<u64>);

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    pub fn apply(&self, a: &u64, b: &u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
            Self::Concat => format!("{a}{b}").parse().unwrap(),
        }
    }
}

pub fn parse_input(input: &str) -> Input {
    let lines = input.split("\n");
    let mut buf = vec![];

    for line in lines {
        let split = line.split(":").collect_vec();
        if split.is_empty() || split.len() < 2 {
            continue;
        }

        let test_value = split
            .first()
            .unwrap()
            .parse::<u64>()
            .expect("valid test value");
        let values = split
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .map(|f| f.parse::<u64>().expect("valid integer"))
            .collect_vec();

        buf.push((test_value, values));
    }

    buf
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(a::solve(input), 1579939);
    }

    #[test]
    fn part_b() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(b::solve(input), 20351745);
    }
}
