use std::cmp::{max, min};

use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let universe = Universe::parse(input.trim_end());

        let mut out = 0;

        for (i, (x, y)) in universe.galaxies.iter().enumerate() {
            for next in (i + 1)..universe.galaxies.len() {
                let (nx, ny) = universe.galaxies[next];

                let mut dx = 0;

                for x in min(*x, nx)..max(*x, nx) {
                    dx += if let Some(true) = universe.expanded_cols.get(x) {
                        2
                    } else {
                        1
                    }
                }
                let mut dy = 0;

                for y in min(*y, ny)..max(*y, ny) {
                    dy += if let Some(true) = universe.expanded_rows.get(y) {
                        2
                    } else {
                        1
                    }
                }
                out += dx + dy;
            }
        }

        Ok(out.to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let universe = Universe::parse(input.trim_end());

        let mut out: i128 = 0;

        for (i, (x, y)) in universe.galaxies.iter().enumerate() {
            for next in (i + 1)..universe.galaxies.len() {
                let (nx, ny) = universe.galaxies[next];

                let mut dx = 0;

                for x in min(*x, nx)..max(*x, nx) {
                    dx += if let Some(true) = universe.expanded_cols.get(x) {
                        1_000_000
                    } else {
                        1
                    }
                }
                let mut dy = 0;

                for y in min(*y, ny)..max(*y, ny) {
                    dy += if let Some(true) = universe.expanded_rows.get(y) {
                        1_000_000
                    } else {
                        1
                    }
                }
                out += dx + dy;
            }
        }

        Ok(out.to_string())
    }
}

struct Universe {
    grid: Vec<Vec<bool>>,
    expanded_rows: Vec<bool>,
    expanded_cols: Vec<bool>,
    galaxies: Vec<(usize, usize)>,
}

impl Universe {
    fn parse(input: &str) -> Self {
        let mut galaxies = vec![];
        let grid: Vec<Vec<bool>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '#' {
                            galaxies.push((x, y));
                        }
                        c == '#'
                    })
                    .collect()
            })
            .collect();
        let mut expanded_rows = vec![];
        let mut expanded_cols = vec![];

        for col in 0..grid[0].len() {
            expanded_cols.push(grid.iter().all(|row| row.get(col) == Some(&false)));
        }

        for row in grid.iter() {
            expanded_rows.push(row.iter().all(|&c| !c))
        }

        Self {
            grid,
            expanded_rows,
            expanded_cols,
            galaxies,
        }
    }
}
