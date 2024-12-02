use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type Report = Vec<usize>;

fn parse(lines: &Vec<String>) -> Vec<Report> {
    lines
        .iter()
        .map(|line| {
            line.split(" ")
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .collect()
        })
        .collect()
}

fn part1(reports: &Vec<Report>) -> usize {
    reports
        .iter()
        .filter(|report| {
            (report.iter().tuple_windows().all(|(a, b)| a < b)
                || report.iter().tuple_windows().all(|(a, b)| a > b))
                && report.iter().tuple_windows().all(|(a, b)| {
                    let diff = (*a as isize).abs_diff(*b as isize);
                    diff <= 3 && diff >= 1
                })
        })
        .count()
}

fn part2(reports: &Vec<Report>) -> usize {
    reports
        .iter()
        .filter_map(|report| {
            (0..report.len())
                .map(|i| [&report[..i], &report[i + 1..]].concat())
                .find(|report| {
                    (report.iter().tuple_windows().all(|(a, b)| a < b)
                        || report.iter().tuple_windows().all(|(a, b)| a > b))
                        && report.iter().tuple_windows().all(|(a, b)| {
                            let diff = (*a as isize).abs_diff(*b as isize);
                            diff <= 3 && diff >= 1
                        })
                })
        })
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
    let parsed = parse(&lines);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}
