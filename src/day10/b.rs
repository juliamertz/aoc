use super::*;
use a::*;

pub fn solve(map: Map) -> usize {
    println!("{}", map);

    let mut score = 0;
    for trailhead in find_trailheads(&map) {
        let mut visited = vec![];
        follow_trail(&mut score, &trailhead, &map, &mut visited, false);
    }

    dbg!(score)
}
