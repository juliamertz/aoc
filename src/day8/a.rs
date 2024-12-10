use std::collections::HashMap;

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

pub fn solve(input: Input) -> u32 {
    // println!("{}", input);

    let mut antennas_by_frequency = HashMap::<char, Vec<Pos>>::new();

    // Find all matching frequencies and their positions
    for (y, line) in input.lines.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if let Tile::Antenna(ch) = tile {
                let pos = (x, y).into();
                let exists = antennas_by_frequency.contains_key(ch);
                if !exists {
                    antennas_by_frequency.insert(*ch, vec![pos]);
                } else {
                    antennas_by_frequency.get_mut(ch).unwrap().push(pos);
                }
            }
        }
    }

    let combinations = antennas_by_frequency
        .iter()
        .map(|(frequency, positions)| (frequency, get_vertices(positions)))
        .collect::<HashMap<_, _>>();

    let mut grid = input.clone();
    let mut ans = 0;

    for (_frequency, combinations) in combinations {
        for positions in combinations {
            let (a, b) = positions.into();
            let x_offset = a.x.abs_diff(b.x);
            let y_offset = a.y.abs_diff(b.y);

            if x_offset == 0 && y_offset == 0 {
                continue;
            }

            if x_offset > a.x || y_offset > a.y {
                continue;
            }

            let mut place_antinode = |pos: Pos| {
                if grid.get(pos).is_some() {
                    grid.set(pos, Tile::Antinode).unwrap();
                    ans += 1;
                }
            };

            let mut colormap = HashMap::new();
            colormap.insert(a, "green");
            colormap.insert(b, "green");

            let (neg_offset_position, pos_offset_position) = if a.y < b.y && a.x < b.x {
                let neg_offset = |pos: Pos| (pos.x - x_offset, pos.y - y_offset);
                let pos_offset = |pos: Pos| (pos.x + x_offset, pos.y + y_offset);

                (neg_offset(a).into(), pos_offset(b).into())
            } else {
                let neg_offset = |pos: Pos| (pos.x - x_offset, pos.y + y_offset);
                let pos_offset = |pos: Pos| (pos.x + x_offset, pos.y - y_offset);

                (pos_offset(a).into(), neg_offset(b).into())
            };

            place_antinode(neg_offset_position);
            place_antinode(pos_offset_position);

            colormap.insert(pos_offset_position, "red");
            colormap.insert(neg_offset_position, "red");

            // println!(
            //     "frequency: {frequency}, a: {a:?}, b: {b:?}, x_offset: {x_offset}, y: {y_offset}"
            // );
            // grid.print_colored(colormap);
        }
    }

    println!("{}", grid);

    dbg!(ans)
}
