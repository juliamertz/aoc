use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    Block(u64),
    Empty,
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Bit::Block(val) => val.to_string(),
                Bit::Empty => ".".into(),
            }
        )
    }
}

pub fn expand_blocks(blocks: &[Block]) -> Vec<Bit> {
    let mut buf = vec![];

    for block in blocks {
        match block {
            Block::File { id, size, .. } => {
                for _ in 0..*size {
                    buf.push(Bit::Block(*id));
                }
            }
            Block::Free(size) => {
                for _ in 0..*size {
                    buf.push(Bit::Empty);
                }
            }
        }
    }

    buf
}

pub fn first_empty(bits: &[Bit]) -> Option<usize> {
    bits.iter()
        .enumerate()
        .find_map(|(i, bit)| if *bit == Bit::Empty { Some(i) } else { None })
}

pub fn compact_bits(mut bits: Vec<Bit>) -> Vec<Bit> {
    // TODO: this seems inefficient
    // edit: it sure is...
    fn is_sorted(bits: &[Bit]) -> bool {
        for bit in bits[first_empty(bits).unwrap()..bits.len() - 1].iter() {
            if *bit != Bit::Empty {
                return false;
            }
        }

        true
    }

    for i in (0..bits.len()).rev() {
        let curr = match bits.get(i) {
            Some(val) => val,
            None => break,
        };

        match *curr {
            Bit::Empty => continue,
            Bit::Block(_) => {
                let empty_idx = first_empty(&bits).expect("atleast one empty index");
                bits[empty_idx] = *curr;
                bits[i] = Bit::Empty;
            }
        }

        if is_sorted(&bits) {
            return bits;
        }
    }

    unreachable!()
}

pub fn checksum(bits: Vec<Bit>) -> u64 {
    let mut ans = 0;

    for (position, bit) in bits.into_iter().enumerate() {
        dbg!(position);
        match bit {
            Bit::Block(id) => ans += id * (position as u64),
            Bit::Empty => continue,
        }
    }

    ans
}

pub fn solve(input: Input) -> u64 {
    let expanded = expand_blocks(&input);
    dbg!(&expanded);
    let compacted = compact_bits(expanded);
    let ans = checksum(compacted);
    dbg!(ans)
}
