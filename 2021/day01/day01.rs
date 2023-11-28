use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn part1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .filter_map(|line| line.parse::<usize>().ok())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn part2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .filter_map(|line| line.parse::<usize>().ok())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
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
    let lines: Vec<_> = io::BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
