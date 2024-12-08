use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type Antennae = HashMap<char, Vec<(isize, isize)>>;
#[derive(Debug)]
struct Grid {
    antennae: Antennae,
    dims: (isize, isize),
}
fn parse(lines: &Vec<String>) -> Grid {
    Grid {
        antennae: lines
            .iter()
            .enumerate()
            .fold(Antennae::new(), |mut antennae, (y, row)| {
                row.chars().enumerate().for_each(|(x, cell)| {
                    if cell == '.' {
                        return;
                    }
                    antennae
                        .entry(cell)
                        .and_modify(|freq_antennae| {
                            freq_antennae.push((x as isize, y as isize));
                        })
                        .or_insert([(x as isize, y as isize)].into_iter().collect());
                });
                antennae
            }),
        dims: (
            lines.first().expect("couldn't get first row").len() as isize,
            lines.len() as isize,
        ),
    }
}

fn calculate_antinodes(
    grid: &Grid,
    antinode_order: isize,
    include_antenna_itself: bool,
) -> Antennae {
    grid.antennae
        .iter()
        .map(|(freq, positions)| {
            (
                *freq,
                positions
                    .iter()
                    .cartesian_product(positions.iter())
                    .filter(|(p1, p2)| *p2 != *p1 || include_antenna_itself)
                    .map(|(p1, p2)| {
                        let diff = (p1.0 - p2.0, p1.1 - p2.1);
                        (1isize..=antinode_order)
                            .map(move |i| (p1.0 + diff.0 * i, p1.1 + diff.1 * i))
                    })
                    .flatten()
                    .filter(|(x, y)| *x >= 0 && *y >= 0)
                    .filter(|(x, y)| *x < grid.dims.0 && *y < grid.dims.1)
                    .collect(),
            )
        })
        .collect()
}

fn part1(grid: &Grid) -> usize {
    let antinodes = calculate_antinodes(&grid, 1, false);
    antinodes
        .iter()
        .map(|(_, positions)| positions.iter())
        .flatten()
        .unique()
        .count()
}

fn part2(grid: &Grid) -> usize {
    let antinodes = calculate_antinodes(&grid, max(grid.dims.0, grid.dims.1), true);
    antinodes
        .iter()
        .map(|(_, positions)| positions.iter())
        .flatten()
        .unique()
        .count()
}

fn main() {
    if env::args().count() != 2 {
        return println!(
            "Usage: {} [path/to/input_file]",
            env::args().next().expect("Couldn't get executable name")
        );
    }
    let input_name: String = env::args().skip(1).next().expect("First argument");
    let f = File::open(input_name).expect("Couldn't open input file");
    let lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .collect();
    let grid = parse(&lines);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
