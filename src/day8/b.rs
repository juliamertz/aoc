// 855/853 answer too low D:

use std::collections::{HashMap, HashSet};

use a::find_antennas;

use super::*;

pub fn get_vertices(points: &[Pos]) -> Vec<Vertex> {
    let mut buf = vec![];

    for i in 0..points.len() {
        for j in 0..points.len() {
            let (curr, next) = (points[i], points[j]);

            let vertex: Vertex = (curr, next).into();
            if !buf.contains(&vertex) {
                buf.push(vertex);
            }
        }
    }

    buf
}

fn place_antinodes_for_vertex(
    vertex: Vertex,
    grid: &mut Input,
    placed_antinodes: &mut HashSet<Pos>,
) {
    let (a, b) = vertex.to_owned().into();
    let x_offset = a.x.abs_diff(b.x);
    let y_offset = a.y.abs_diff(b.y);

    if x_offset == 0 && y_offset == 0 {
        dbg!("OFFSETS ARE 0");
        return;
    }

    let mut colormap = HashMap::new();
    colormap.insert(a, "green");
    colormap.insert(b, "green");

    let mut place_antinode = |pos: Pos| {
        if let Some(tile) = grid.get(&pos) {
            // if *tile != Tile::Antinode {
            if *tile == Tile::Empty {
                grid.set(pos, Tile::Antinode).unwrap();
            }
            colormap.insert(pos, "red");
            placed_antinodes.insert(pos);
            return Some(pos);
        }
        // }
        None
    };

    fn place_antinodes(
        x_offset: usize,
        y_offset: usize,
        calc: impl Fn(usize, usize) -> Option<Pos>,
        place: &mut impl FnMut(Pos) -> Option<Pos>,
    ) {
        let mut i = 1;
        loop {
            let x_offset = x_offset * i;
            let y_offset = y_offset * i;

            let pos = match calc(x_offset, y_offset) {
                Some(val) => val,
                None => break,
            };
            // let pos = (b.x + x_offset, b.y + y_offset).into();
            match place(pos) {
                Some(_) => {}
                None => break,
            };
            i += 1;
        }
    }

    if a.y < b.y && a.x < b.x {
        if y_offset <= a.y && x_offset <= a.x {
            place_antinodes(
                x_offset,
                y_offset,
                |x_offset, y_offset| {
                    Some((a.x.checked_sub(x_offset)?, a.y.checked_sub(y_offset)?).into())
                },
                &mut place_antinode,
            );
        }

        place_antinodes(
            x_offset,
            y_offset,
            |x_offset, y_offset| Some((b.x + x_offset, b.y + y_offset).into()),
            &mut place_antinode,
        );
    } else {
        if y_offset <= a.y {
            place_antinodes(
                x_offset,
                y_offset,
                |x_offset, y_offset| Some((a.x + x_offset, a.y.checked_sub(y_offset)?).into()),
                &mut place_antinode,
            );
        }

        if x_offset <= b.x {
            place_antinodes(
                x_offset,
                y_offset,
                |x_offset, y_offset| Some((b.x.checked_sub(x_offset)?, b.y + y_offset).into()),
                &mut place_antinode,
            );
        }
    };

    // println!(
    //             "frequency: {frequency}, a: {a:?}, b: {b:?}, x_offset: {x_offset}, y: {y_offset}, count: {}", placed_antinodes.len()
    //         );
    grid.print_colored(&colormap);
}

pub fn solve(input: Input) -> u32 {
    let antennas_by_frequency = find_antennas(&input);
    let combinations = antennas_by_frequency
        .iter()
        .map(|(frequency, positions)| (frequency, get_vertices(positions)))
        .collect::<HashMap<_, _>>();

    dbg!(&combinations);

    let mut grid = input.clone();

    let mut placed_antinodes = HashSet::<Pos>::new();

    for (_frequency, combinations) in combinations {
        for vertex in combinations {
            placed_antinodes.insert(vertex.0);
            placed_antinodes.insert(vertex.1);
            place_antinodes_for_vertex(vertex, &mut grid, &mut placed_antinodes);
        }
    }

    println!("{}", grid);
    let ans = placed_antinodes.len();

    dbg!(ans as u32)
}
