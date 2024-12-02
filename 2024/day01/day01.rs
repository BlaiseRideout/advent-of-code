use itertools::{sorted, Itertools};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn parse(lines: &Vec<String>) -> [Vec<usize>; 2] {
    lines
        .iter()
        .fold([vec![], vec![]], |vecs, line| {
            line.split_whitespace()
                .zip(vecs)
                .map(|(x, vec)| [vec, vec![x.parse().expect("couldn't parse num")]].concat())
                .take(2)
                .collect::<Vec<_>>()
                .try_into()
                .expect("couldn't unwrap two values")
        })
        .map(|list| sorted(list).collect())
}

fn part1(lists: &[Vec<usize>; 2]) -> usize {
    lists[0]
        .iter()
        .zip(lists[1].iter())
        .map(|(x, y)| (*x as isize).abs_diff(*y as isize))
        .sum()
}

fn part2(lists: &[Vec<usize>; 2]) -> usize {
    let counts = lists[1]
        .iter()
        .fold(HashMap::<usize, usize>::new(), |mut counts, x| {
            *counts.entry(*x).or_insert(0) += 1;
            counts
        });
    lists[0]
        .iter()
        .map(|x| x * counts.get(x).unwrap_or(&0))
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
    let parsed = parse(&lines);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}
