use super::*;

pub fn find_trailheads(map: &Map) -> Vec<Pos> {
    let mut buf = vec![];

    for (y, line) in map.lines.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                buf.push((x, y).into());
            }
        }
    }

    buf
}

/// Returns vec of max height tiles reachable from this position
pub fn follow_trail(
    score: &mut usize,
    pos: &Pos,
    map: &Map,
    visited: &mut Vec<Pos>,
    filter_dupes: bool,
) {
    for dir in Direction::ALL {
        let dir = pos.to(dir);
        let curr = map.get(pos).unwrap();
        if *curr == 9 {
            *score += 1;
            break;
        }

        let next_pos = match dir {
            Some(pos) => pos,
            None => continue,
        };
        let height = match map.get(&next_pos) {
            Some(height) => height,
            None => continue,
        };

        if visited.contains(&next_pos) && filter_dupes {
            continue;
        }

        visited.push(*pos);

        if *height == curr + 1 {
            visited.push(next_pos);
            follow_trail(score, &next_pos, map, visited, filter_dupes);
        }
    }
}

pub fn solve(map: Map) -> usize {
    println!("{}", map);

    let mut score = 0;
    for trailhead in find_trailheads(&map) {
        let mut visited = vec![];
        follow_trail(&mut score, &trailhead, &map, &mut visited, true);
    }

    dbg!(score)
}
