use super::*;

pub fn solve(mut input: Input) -> u64 {
    let mut sum = 0;

    println!("{input}");

    for pos in input.positions_iter() {
        if let Some(Tile::Empty) = input.get(&pos) {
            continue;
        }

        let adjacent_positions = [
            pos.to(Direction::Left),
            pos.to(Direction::Right),
            pos.to(Direction::Up),
            pos.to(Direction::Down),
            pos.to(Direction::Left)
                .and_then(|pos| pos.to(Direction::Down)),
            pos.to(Direction::Left)
                .and_then(|pos| pos.to(Direction::Up)),
            pos.to(Direction::Right)
                .and_then(|pos| pos.to(Direction::Down)),
            pos.to(Direction::Right)
                .and_then(|pos| pos.to(Direction::Up)),
        ]
        .into_iter()
        .filter(|pos| pos.map(|ref pos| input.get(pos)).flatten().is_some());

        let roll_positions = adjacent_positions
            .into_iter()
            .filter_map(|pos| {
                pos.and_then(|pos| {
                    let tile = input.get(&pos);
                    if matches!(tile, Some(&Tile::Roll)) {
                        Some(pos)
                    } else {
                        None
                    }
                })
            })
            .collect_vec();

        if roll_positions.len() < 4 {
            sum += 1;

            let as_mut = input.get_mut(&pos).unwrap();
            *as_mut = Tile::Empty;
        };
    }

    println!("{input}");

    sum
}
