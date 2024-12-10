use super::*;

pub type Guard = Option<(Pos, Orientation)>;

pub fn find_guard(board: &Board) -> Option<(Pos, Orientation)> {
    for (y, line) in board.lines.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::Guard(o) => return Some(((x, y).into(), *o)),
                _ => continue,
            };
        }
    }

    None
}

pub fn tick(mut board: Board, guard: Guard) -> Option<(Board, Guard)> {
    let (pos, orientation) = guard?;

    let orientation = match board.get(orientation.move_pos(pos)?) {
        Some(&Tile::Obstacle(_)) => orientation.rotate_right(),
        _ => orientation,
    };

    let next_pos = orientation.move_pos(pos)?;
    board.set(next_pos, Tile::Guard(orientation)).ok()?;
    board.set(pos, Tile::Visited).ok()?;

    let guard = Some((next_pos, orientation));
    Some((board, guard))
}

pub fn walk_path(board: Board) -> (Board, Guard) {
    let mut board = board;
    let mut guard = find_guard(&board);

    while let Some((next, g)) = tick(board.clone(), guard) {
        board = next;
        guard = g;
    }

    (board, guard)
}

pub fn solve(board: Board) -> usize {
    walk_path(board)
        .0
        .lines
        .into_iter()
        .flatten()
        .filter(|tile| tile == &Tile::Visited)
        .count()
        + 1
}
