use super::*;

fn check_xmas(input: &Input, pos: Pos) -> Option<()> {
    let (x, y) = pos;

    let top_right = input.get((x + 1, y + 1));
    let top_left = input.get((x.checked_sub(1)?, y + 1));
    let bottom_right = input.get((x + 1, y.checked_sub(1)?));
    let bottom_left = input.get((x.checked_sub(1)?, y.checked_sub(1)?));

    let check = |a, b| match (a, b) {
        (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M')) => Some(()),
        _ => None,
    };

    check(top_left, bottom_right).and(check(bottom_left, top_right))
}

pub fn solve(input: Input) -> u32 {
    let mut ans = 0;

    for (y, line) in input.content.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == 'A' && check_xmas(&input, (x, y)).is_some() {
                ans += 1;
            }
        }
    }

   dbg!(ans) 
}
