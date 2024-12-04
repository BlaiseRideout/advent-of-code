use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect()).collect()
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn reverse<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    v.into_iter()
        .map(|row| row.into_iter().rev().collect())
        .collect()
}

fn flip<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    v.into_iter()
        .map(|row| row.into_iter().collect())
        .rev()
        .collect()
}

fn count_horizontal(grid: &Vec<Vec<char>>, needle: &str) -> usize {
    grid.iter()
        .map(|row| {
            (0..=row.len() - needle.len())
                .filter(|i| {
                    row.into_iter()
                        .skip(*i)
                        .take(needle.len())
                        .zip(needle.chars())
                        .all(|(x, y)| *x == y)
                        || row
                            .into_iter()
                            .skip(*i)
                            .take(needle.len())
                            .rev()
                            .zip(needle.chars())
                            .all(|(x, y)| *x == y)
                })
                .count()
        })
        .sum()
}

fn count_diagonal(grid: &Vec<Vec<char>>, needle: &str) -> usize {
    (0..=grid.len() - needle.len())
        .map(|y| {
            (0..=grid[0].len() - needle.len())
                .filter(|x| {
                    (0..needle.len()).all(|i| {
                        grid[y + i][*x + i] == needle.chars().nth(i).expect("Couldn't get char")
                    })
                })
                .count()
        })
        .sum()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!(
        "{}",
        grid.iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n")
    );
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    let needle = "XMAS";
    let transposed = transpose2(grid.clone());
    let reversed = reverse(grid.clone());
    let flipped = flip(grid.clone());
    let flipped_reversed = flip(reversed.clone());
    count_horizontal(grid, needle)
        + count_horizontal(&transposed, needle)
        + count_diagonal(grid, needle)
        + count_diagonal(&reversed, needle)
        + count_diagonal(&flipped, needle)
        + count_diagonal(&flipped_reversed, needle)
}

fn count_xmas(grid: &Vec<Vec<char>>) -> usize {
    let needle = "MAS";
    (1..grid.len() - 1)
        .map(|y| {
            (1..grid[0].len() - 1)
                .filter(|x| {
                    let mid = grid[y][*x];
                    let lower_right = grid[y + 1][x + 1];
                    let upper_left = grid[y - 1][x - 1];
                    let upper_right = grid[y - 1][x + 1];
                    let lower_left = grid[y + 1][x - 1];
                    [
                        [upper_right, mid, lower_left]
                            .iter()
                            .zip(needle.chars())
                            .all(|(x, y)| *x == y),
                        [upper_left, mid, lower_right]
                            .iter()
                            .zip(needle.chars())
                            .all(|(x, y)| *x == y),
                        [upper_left, mid, lower_right]
                            .iter()
                            .zip(needle.chars().rev())
                            .all(|(x, y)| *x == y),
                        [upper_right, mid, lower_left]
                            .iter()
                            .zip(needle.chars().rev())
                            .all(|(x, y)| *x == y),
                    ]
                    .iter()
                    .filter(|b| **b)
                    .count()
                        == 2
                })
                .count()
        })
        .sum()
}
fn part2(grid: &Vec<Vec<char>>) -> usize {
    count_xmas(grid)
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
    let parsed = parse(&lines);
    print_grid(&parsed);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}
