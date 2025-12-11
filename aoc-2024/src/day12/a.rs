use std::collections::HashMap;

use super::*;

// day 12
// Plot is a single tile
// A region is a collection of plots growing the same type
// perimiter is amount of plot sides that don't touch any other plots

pub type Plot = Pos;

// Returns region positions and total perimiter for this region
pub fn scan_region(plot: Plot, garden: &Garden) -> (Vec<Plot>, usize) {
    let mut plots = vec![];
    let mut perimiter = 0;
    let mut colors = HashMap::new();

    dbg!(&plot);

    fn walk_crop(
        plot: Pos,
        garden: &Garden,
        buf: &mut Vec<Plot>,
        colors: &mut HashMap<Pos, &str>,
        perimiter: &mut usize,
    ) {
        let crop = garden.get(&plot).unwrap();
        dbg!(&crop);
        buf.push(plot);
        let mut has_neighbour = false;

        for dir in Direction::ALL {
            let next_plot = match plot.to(dir) {
                Some(val) => val,
                None => {
                    // If index if out of range it means it's at a border thus we add 1 to perimiter
                    *perimiter += 1;
                    continue;
                }
            };
            let next_crop = match garden.get(&next_plot) {
                Some(crop) => crop,
                None => {
                    // Same goes for when encountering a different crop type
                    *perimiter += 1;
                    continue;
                }
            };


            if next_crop != crop {
                continue;
            }
            has_neighbour = true;

            if buf.contains(&next_plot) {
                continue;
            }


            colors.insert(plot, "red");
            colors.insert(next_plot, "red");

            walk_crop(next_plot, garden, buf, colors, perimiter);
        }

        if !has_neighbour {
            dbg!(plot, "no neighbours");
            *perimiter += 4;
        }

        garden.print_colored(colors);
    }

    walk_crop(plot, garden, &mut plots, &mut colors, &mut perimiter);

    (plots, perimiter)
}

pub fn solve(garden: Garden) -> u32 {
    let mut scanned_plots = vec![];
    let mut p = 0;
    let mut colors = HashMap::new();

    for plot in garden.positions_iter() {
        if scanned_plots.contains(&plot) {
            continue;
        }
        colors.insert(plot, "green");
        garden.print_colored(&colors);
        dbg!(plot);
        let (area, perimiter) = scan_region(plot, &garden);
        dbg!(&area, perimiter);
        p += perimiter * area.len();
        scanned_plots.extend(area);
    }

    dbg!(p);

    0
}
