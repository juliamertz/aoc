use std::ops::{Add, Sub};

use super::*;

#[derive(Debug)]
pub struct Dial {
    pos: u64,
    pub naive_count: u64,
    pub full_count: u64,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            pos: 50,
            naive_count: 0,
            full_count: 0,
        }
    }
}

impl Add<u64> for Dial {
    type Output = Dial;

    fn add(mut self, rhs: u64) -> Self::Output {
        for _ in 0..rhs {
            if self.pos == 99 {
                self.pos = 0;
            } else {
                self.pos += 1
            }
            if self.pos == 0 {
                self.full_count += 1
            }
        }
        if self.pos == 0 {
            self.naive_count += 1
        }

        self
    }
}

impl Sub<u64> for Dial {
    type Output = Dial;

    fn sub(mut self, rhs: u64) -> Self::Output {
        for _ in 0..rhs {
            if self.pos == 0 {
                self.pos = 99;
            } else {
                self.pos -= 1
            }
            if self.pos == 0 {
                self.full_count += 1
            }
        }
        if self.pos == 0 {
            self.naive_count += 1
        }

        self
    }
}

impl Dial {
    pub fn apply_rotations(mut self, rotations: Input) -> Self {
        for rotation in rotations {
            self = match rotation {
                Rotation::Left(amount) => self.sub(amount),
                Rotation::Right(amount) => self.add(amount),
            }
        }
        self
    }
}

pub fn solve(input: Input) -> u64 {
    let dial = Dial::default().apply_rotations(input);
    dial.naive_count
}
