use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn parse(lines: &Vec<String>) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if ('0'..='9').contains(&c) {
                        c
                    } else {
                        ('0' as u8 + 20) as char
                    }
                })
                .map(|c| c as usize - '0' as usize)
                .collect()
        })
        .collect()
}

fn print_grid(grid: &Vec<Vec<usize>>) {
    println!(
        "{}",
        grid.iter()
            .map(|line| line
                .iter()
                .map(|c| if (0..=9).contains(c) {
                    c.to_string()
                } else {
                    ".".to_string()
                })
                .join(""))
            .join("\n")
    );
}

fn find_trailheads(grid: &Vec<Vec<usize>>) -> HashSet<(usize, usize)> {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(move |(x, _)| (x, y))
        })
        .flatten()
        .collect()
}

fn score_trailhead(grid: &Vec<Vec<usize>>, trailhead: &(usize, usize)) -> (usize, usize) {
    let mut nodes = [vec![*trailhead]]
        .into_iter()
        .collect::<VecDeque<Vec<(usize, usize)>>>();
    let mut visited_paths = HashSet::<Vec<(usize, usize)>>::new();
    let mut score = 0;
    while let Some(path) = nodes.pop_back() {
        if let Some((x, y)) = path.last().cloned() {
            let node_val = grid[y][x];
            visited_paths.insert(path.clone());
            if node_val == 9 {
                score += 1;
            }

            let mut new_paths = (-1..=1isize)
                .cartesian_product(-1..=1isize)
                .filter(|(x, y)| x != y && *x != -y)
                .map(|(deltax, deltay)| {
                    (x as isize + deltax as isize, y as isize + deltay as isize)
                })
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .filter(|(x, y)| *y < grid.len() && *x < grid[0].len())
                .filter(|(x, y)| node_val + 1 == grid[*y][*x])
                .map(|p| path.iter().cloned().chain([p].into_iter()).collect_vec())
                .collect::<Vec<Vec<(usize, usize)>>>();

            new_paths.retain(|node| !visited_paths.contains(node.as_slice()));
            nodes.extend(new_paths.into_iter());
        }
    }

    (
        visited_paths
            .iter()
            .filter_map(|v| v.last())
            .filter(|(x, y)| grid[*y][*x] == 9)
            .unique()
            .count(),
        visited_paths
            .iter()
            .filter_map(|v| v.last())
            .filter(|(x, y)| grid[*y][*x] == 9)
            .count(),
    )
}

fn part1(grid: &Vec<Vec<usize>>) -> usize {
    let trailheads = find_trailheads(grid);
    trailheads
        .iter()
        .map(|trailhead| score_trailhead(grid, trailhead).0)
        .sum()
}

fn part2(grid: &Vec<Vec<usize>>) -> usize {
    let trailheads = find_trailheads(grid);
    trailheads
        .iter()
        .map(|trailhead| score_trailhead(grid, trailhead).1)
        .sum()
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
    print_grid(&grid);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
