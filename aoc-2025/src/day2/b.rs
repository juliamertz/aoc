use super::*;

fn is_invalid(num: u64) -> bool {
    let bytes = num.to_string().into_bytes();
    let split_lengths = 1..bytes.len();
    // dbg!(&split_lengths);

    for len in split_lengths {
        if bytes.len() % len == 0 {
            // dbg!(num, len);
            let mut parts = bytes.chunks(len);
            let pattern = parts.next().unwrap();
            if parts.len() == 0 {
                continue;
            }

            if parts.all(|part| part == pattern) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::day2::b::is_invalid;

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid(1010));
        assert!(is_invalid(1111111));
        assert!(is_invalid(2121212121));
    }
}

fn sum_invalid(range: Range) -> u64 {
    let mut buf = 0;
    if range.start < range.end {
        for idx in range.start..=range.end {
            if is_invalid(idx) {
                buf += idx;
            }
        }
    }
    buf
}

pub fn solve(input: Input) -> u64 {
    input
        .into_iter()
        .map(sum_invalid)
        .collect::<Vec<_>>()
        .iter()
        .sum()
}
