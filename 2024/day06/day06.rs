use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type Grid = Vec<Vec<char>>;

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn print_grid(grid: &Grid, visited_poss: &HashSet<(usize, usize)>) {
    println!(
        "{}",
        grid.iter()
            .enumerate()
            .map(|(y, row)| row
                .iter()
                .enumerate()
                .map(|(x, cell)| if visited_poss.contains(&(x, y)) {
                    'X'
                } else {
                    *cell
                })
                .collect::<String>())
            .join("\n")
    );
}

fn find_start_pos(grid: &Grid) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .filter_map(|(y, row)| {
            if let Some((x, _)) = row.iter().enumerate().find(|(_, cell)| **cell == '^') {
                Some((x, y))
            } else {
                None
            }
        })
        .take(1)
        .exactly_one()
        .expect("couldn't find start pos")
}

fn walk_grid(
    grid: &Grid,
    start_pos: (usize, usize),
    test_pos: Option<(usize, usize)>,
) -> (HashSet<((usize, usize), (isize, isize))>, bool) {
    let mut pos = start_pos.clone();
    let mut dir: (isize, isize) = (0, -1);

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut visited_poss = HashSet::<((usize, usize), (isize, isize))>::new();
    let in_bounds =
        |pos: (isize, isize)| pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height;

    let mut left_bounds = false;
    loop {
        visited_poss.insert((pos, dir));
        let new_pos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
        if in_bounds(new_pos) {
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

            if visited_poss.contains(&(new_pos, dir)) {
                break;
            }
            if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' || Some(new_pos) == test_pos {
                dir = (dir.1 * -1, dir.0);
                visited_poss.insert((pos, dir));
            } else {
                pos = new_pos;
            }
        } else {
            left_bounds = true;
            break;
        }
    }

    (visited_poss, left_bounds)
}

fn part1(grid: &Grid) -> usize {
    let start_pos = find_start_pos(grid);

    let (visited_poss, _) = walk_grid(grid, start_pos, None);
    let visited_poss = visited_poss
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<(usize, usize)>>();
    print_grid(grid, &visited_poss);
    visited_poss.len()
}

fn part2(grid: &Grid) -> usize {
    let start_pos = find_start_pos(grid);

    let (visited_poss, _) = walk_grid(grid, start_pos, None);
    let visited_poss = visited_poss
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<(usize, usize)>>();

    visited_poss
        .iter()
        .filter(|(x, y)| (*x, *y) != start_pos)
        .filter(|(x, y)| !walk_grid(grid, start_pos, Some((*x, *y))).1)
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
