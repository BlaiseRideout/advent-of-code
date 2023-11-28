use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn num_increasing<I: Iterator<Item = usize>>(a: I) -> usize {
    a.tuple_windows().filter(|(a, b)| b > a).count()
}

fn part1(lines: &Vec<usize>) -> usize {
    num_increasing(lines.iter().cloned())
}

fn part2(lines: &Vec<usize>) -> usize {
    num_increasing(lines.windows(3).map(|v| v.iter().sum()))
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

    let parsed = lines
        .iter()
        .map(|line| line.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}
