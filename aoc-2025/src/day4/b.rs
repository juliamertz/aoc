use super::*;

fn remove_rolls(grid: &mut Grid<Tile>) -> u64 {
    let mut sum = 0;

    for pos in grid.positions_iter() {
        // TODO: upgrade to rust with if let support
        if let Some(tile) = grid.get(&pos) {
            if tile == &Tile::Empty {
                continue;
            }
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
        .filter(|pos| pos.map(|ref pos| grid.get(pos)).flatten().is_some());

        let roll_positions = adjacent_positions
            .into_iter()
            .filter_map(|pos| {
                pos.and_then(|pos| {
                    let tile = grid.get(&pos);
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

            let as_mut = grid.get_mut(&pos).unwrap();
            *as_mut = Tile::Empty;
        };
    }

    sum
}

pub fn solve(mut input: Input) -> u64 {
    let mut sum = 0;

    loop {
        let removed = remove_rolls(&mut input);
        dbg!(&removed);
        if removed == 0 {
            break;
        }

        sum += removed;
    }

    println!("{input}");

    sum
}
