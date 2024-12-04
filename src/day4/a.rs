// this was a waste of time
use super::*;
use strum::IntoEnumIterator;

type WordPos = Vec<Pos>;

#[derive(strum_macros::EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagTopLeft,
    DiagTopRight,
    DiagBottomLeft,
    DiagBottomRight,
}

impl Direction {
    fn step(&self, pos: &Pos) -> Option<Pos> {
        let (x, y) = pos;
        Some(match self {
            Self::Up => (*x, y.checked_sub(1)?),
            Self::Down => (*x, y + 1),
            Self::Left => (x.checked_sub(1)?, *y),
            Self::Right => (x + 1, *y),
            Self::DiagTopLeft => (x.checked_sub(1)?, y.checked_sub(1)?),
            Self::DiagTopRight => (x + 1, y.checked_sub(1)?),
            Self::DiagBottomLeft => (x.checked_sub(1)?, y + 1),
            Self::DiagBottomRight => (x + 1, y + 1),
        })
    }
}

fn match_word_from_positions(indeces: &WordPos, input: &Input, word: &str) -> bool {
    for (i, pos) in indeces.iter().enumerate() {
        if input.get(*pos) != Some(&word.chars().nth(i).unwrap()) {
            return false;
        }
    }

    true
}

fn search_word_2d(
    pos: (usize, usize),
    direction: Direction,
    word: &str,
    input: &Input,
) -> Option<WordPos> {
    let mut indeces: Vec<Pos> = vec![];
    let word_chars = word.split("").filter(|ch| !ch.is_empty());

    for _ in word_chars {
        let pos = match indeces.last() {
            Some(value) => value,
            None => {
                indeces.push(pos);
                continue;
            }
        };

        indeces.push(direction.step(pos)?);
    }

    if match_word_from_positions(&indeces, input, word) {
        Some(indeces)
    } else {
        None
    }
}

pub fn solve(input: Input) -> usize {
    let mut indeces: Vec<WordPos> = vec![];

    for (line_no, line) in input.content.iter().enumerate() {
        for (ch_no, _ch) in line.iter().enumerate() {
            for direction in Direction::iter() {
                if let Some(res) = search_word_2d((ch_no, line_no), direction, "XMAS", &input) {
                    indeces.push(res);
                }
            }
        }
    }

    dbg!(indeces.len())
}
