use super::*;
use a::{find_guard, walk_path, Guard};
use std::collections::HashSet;

// I'm truly ashamed of what i did here today

pub fn tick(mut board: Board, guard: Guard) -> Option<(Board, Guard)> {
    let (pos, orientation) = guard?;

    let mut done_rotating = false;
    let mut next_orientation = orientation;
    while !done_rotating {
        next_orientation = match board.get(next_orientation.move_pos(pos)?) {
            Some(&Tile::Obstacle(_)) => next_orientation.rotate_right(),
            _ => {
                done_rotating = true;
                next_orientation
            }
        };
    }

    let next_pos = next_orientation.move_pos(pos)?;
    board.set(next_pos, Tile::Guard(next_orientation)).ok()?;
    if next_orientation != orientation {
        board.set(pos, Tile::Corner).ok()?;
    } else {
        board.set(pos, Tile::VisitedFacing(next_orientation)).ok()?;
    }

    let guard = Some((next_pos, next_orientation));
    Some((board, guard))
}

fn is_loop(mut board: Board) -> bool {
    let mut positions: HashSet<(Pos, Orientation)> = HashSet::new();
    let mut last = match find_guard(&board) {
        Some(val) => val,
        None => return false,
    };

    loop {
        match tick(board.clone(), Some(last)) {
            Some((next, guard)) => {
                board = next;
                // println!("{}", board);
                if positions.contains(&guard.unwrap()) {
                    return true;
                }

                positions.insert(guard.unwrap());
                last = guard.unwrap()
            }
            None => return false,
        }
    }
}

pub fn solve(board: Board) -> usize {
    let mut possible_positions: Vec<Pos> = vec![];

    // add each visited tile as possible position for new obstruction
    let (walked_board, _) = walk_path(board.clone());
    for (y, line) in walked_board.lines.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::Visited | Tile::Guard(_) => {
                    possible_positions.push((x, y).into());
                }
                _ => continue,
            };
        }
    }

    // Guards starting point can be ignored as he will notice a new obstruction
    let start = find_guard(&board);
    possible_positions = possible_positions
        .into_iter()
        .filter(|p| *p != start.unwrap().0)
        .collect_vec();

    parallel_accumulate(possible_positions, 0, move |data, acc| {
        for pos in data {
            let mut board = board.clone();
            board.set(pos, Tile::Obstacle(true)).unwrap();

            if is_loop(board) {
                let mut count = acc.lock().unwrap();
                *count += 1;
                dbg!(*count);
            }
        }
    })
}
