use std::collections::{HashMap, HashSet};

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

pub fn find_antennas(input: &Input) -> HashMap<char, Vec<Pos>> {
    let mut buf = HashMap::new();

    // Find all matching frequencies and their positions
    for (y, line) in input.lines.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if let Tile::Antenna(ch) = tile {
                let pos = (x, y).into();
                let exists = buf.contains_key(ch);
                if !exists {
                    buf.insert(*ch, vec![pos]);
                } else {
                    buf.get_mut(ch).unwrap().push(pos);
                }
            }
        }
    }

    buf
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

    for (frequency, combinations) in combinations {
        for vertex in combinations {
            let (a, b) = vertex.to_owned().into();
            let x_offset = a.x.abs_diff(b.x);
            let y_offset = a.y.abs_diff(b.y);

            if x_offset == 0 && y_offset == 0 {
                dbg!("OFFSETS ARE 0");
                continue;
            }

            let mut colormap = HashMap::new();
            colormap.insert(a, "green");
            colormap.insert(b, "green");

            let mut place_antinode = |pos: Pos| {
                if let Some(tile) = grid.get(&pos) {
                    if *tile != Tile::Antinode {
                        if *tile == Tile::Empty {
                            grid.set(pos, Tile::Antinode).unwrap();
                        }
                        colormap.insert(pos, "red");
                        // ans += 1;
                        placed_antinodes.insert(pos);
                    }
                }
            };

            if a.y < b.y && a.x < b.x {
                if y_offset <= a.y && x_offset <= a.x {
                    let neg = (a.x - x_offset, a.y - y_offset).into();
                    place_antinode(neg);
                }

                let pos = (b.x + x_offset, b.y + y_offset).into();
                place_antinode(pos);
            } else {
                if y_offset <= a.y {
                    let neg = (a.x + x_offset, a.y - y_offset).into();
                    place_antinode(neg);
                }

                if x_offset <= b.x {
                    let pos = (b.x - x_offset, b.y + y_offset).into();
                    place_antinode(pos);
                }
            };

            println!(
                "frequency: {frequency}, a: {a:?}, b: {b:?}, x_offset: {x_offset}, y: {y_offset}, count: {}", placed_antinodes.len()
            );
            grid.print_colored(&colormap);
        }
    }

    println!("{}", grid);
    let ans = placed_antinodes.len();

    dbg!(ans as u32)
}
