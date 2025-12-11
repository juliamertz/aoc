pub mod a;
pub mod b;

pub use super::*;

#[derive(Debug)]
pub struct Id<'a>(pub &'a str);

impl Id<'_> {
    pub fn is_valid(&self) -> bool {
        let bytes = self.0.as_bytes();
        if bytes[0] == b'0' {
            return false;
        }

        let (lhs, rhs) = bytes.split_at(bytes.len() / 2);
        lhs != rhs
    }
}

impl<'a> From<&'a str> for Id<'a> {
    fn from(value: &'a str) -> Self {
        Self(value.trim())
    }
}

#[derive(Debug)]
pub struct Range<'a> {
    start: Id<'a>,
    end: Id<'a>,
}

pub type Input<'a> = Vec<Range<'a>>;

pub fn parse_input<'a>(input: &'a str) -> Input<'a> {
    input
        .split(",")
        .map(|item| {
            item.split_once("-")
                .map(|(start, end)| Range {
                    start: start.into(),
                    end: end.into(),
                })
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(a::solve(input), 1055);
    }

    #[test]
    fn part_b() {
        let input = parse_input(include_str!("./input.txt"));
        assert_eq!(b::solve(input), 6386);
    }
}
