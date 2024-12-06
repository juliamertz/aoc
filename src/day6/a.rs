
use super::*;

pub fn find_guard(board: &Board) -> Option<(Pos, Orientation)> {
    for (y, line) in board.lines.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::Guard(o) => return Some(((x, y), *o)),
                _ => continue,
            };
        }
    }

    None
}

pub fn tick(mut board: Board) -> Option<Board> {
    let (pos, orientation) = find_guard(&board)?;

    let orientation = if board.get(orientation.move_pos(pos)) == Some(&Tile::Obstacle) {
        orientation.rotate_right()
    } else {
        orientation
    };

    board
        .set(orientation.move_pos(pos), Tile::Guard(orientation))
        .ok()?;
    board.set(pos, Tile::Visited).ok()?;

    Some(board)
}

pub fn solve(board: Board) -> usize {
    let mut board = board;

    loop {
        match tick(board.clone()) {
            Some(next) => {
                board = next;
            }
            None => break,
        }
    }

    board
        .lines
        .into_iter()
        .flatten()
        .filter(|tile| tile == &Tile::Visited)
        .count()
        + 1
}
