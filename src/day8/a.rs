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
    println!("{}", input);

    let mut antennas_by_frequency = HashMap::<char, Vec<Pos>>::new();

    // Find all matching frequencies and their positions
    for (y, line) in input.lines.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if let Tile::Antenna(ch) = tile {
                let pos = (x, y);
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

    dbg!(&combinations);

    let mut grid = input.clone();
    let mut ans = 0;

    // calculate x/y offset
    for (frequency, combinations) in combinations {
        // i think theres some silly mistake in here
        for positions in combinations {
            let (a, b) = positions.into();
            let x_offset = a.0.abs_diff(b.0);
            let y_offset = a.1.abs_diff(b.1);

            if x_offset == 0 && y_offset == 0 {
                continue;
            }

            dbg!(frequency, a, b, x_offset, y_offset);

            if x_offset > a.0 || y_offset > a.1 {
                continue;
            }

            let neg_offset_postion = (a.0 - x_offset, a.1 - y_offset);
            if grid.get(neg_offset_postion).is_some() {
                grid.set(neg_offset_postion, Tile::Antinode).unwrap();
                ans += 1;
            }

            let pos_offset_postion = (b.0 + x_offset, b.1 + y_offset);
            if grid.get(pos_offset_postion).is_some() {
                grid.set(pos_offset_postion, Tile::Antinode).unwrap();
                ans += 1;
            }
        }
    }

    println!("{}", grid);
    println!("ans = {ans}");

    // place antinode at both antenna's + offset (if in bounds)

    10
}
