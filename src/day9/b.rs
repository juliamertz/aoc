use super::*;
use a::*;

/// Returns index and following free bits
pub fn first_sized_empty(bits: &[Bit], n: usize) -> Option<(usize, usize)> {
    let mut idx = None;
    let mut free = 0;

    for (i, bit) in bits.iter().enumerate() {
        if *bit == Bit::Empty {
            free += 1;
            if idx.is_none() {
                idx = Some(i);
            }
        } else {
            if idx.is_some() && free >= n {
                break;
            }
            free = 0;
            idx = None;
        }
    }

    if free < n {
        None
    } else {
        idx.map(|v| (v, free))
    }
}

pub fn compact_bits(mut bits: Vec<Bit>, blocks: &[Block]) -> Vec<Bit> {
    let mut idx = blocks.len() - 1;

    loop {
        let (id, block_size, block_start, block_end) = match blocks[idx] {
            Block::File {
                id,
                size,
                start,
                end,
            } => (id, size, start, end),
            Block::Free(_) => {
                idx -= 1;
                continue;
            }
        };

        match first_sized_empty(&bits, block_size) {
            Some((start, _)) => {
                if start > block_start {
                    if idx != 0 {
                        idx -= 1;
                    } else {
                        break;
                    }
                    continue;
                }

                for bit in bits[start..(start + block_size)].iter_mut() {
                    *bit = Bit::Block(id);
                }
                for bit in bits[block_start..block_end].iter_mut() {
                    *bit = Bit::Empty;
                }
            }
            None => {
                idx -= 1;
                continue;
            }
        }

        if idx == 0 {
            break;
        }
        idx -= 1;
    }

    bits
}

pub fn solve(blocks: Input) -> u64 {
    let bits = expand_blocks(&blocks);
    let compacted = compact_bits(bits, &blocks);
    let ans = checksum(compacted);
    dbg!(ans)
}
