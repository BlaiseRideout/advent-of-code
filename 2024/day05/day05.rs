use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type PageOrdering = (usize, usize);

fn parse(lines: &Vec<String>) -> (HashSet<PageOrdering>, Vec<Vec<usize>>) {
    let (rules, updates) = lines.split(String::is_empty).collect_tuple().unwrap();
    (
        rules
            .iter()
            .map(|line| {
                line.split("|")
                    .map(str::parse::<usize>)
                    .filter_map(Result::ok)
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
        updates
            .iter()
            .map(|line| {
                line.split(",")
                    .map(str::parse::<usize>)
                    .filter_map(Result::ok)
                    .collect()
            })
            .collect(),
    )
}

fn update_is_ordered(update: &Vec<usize>, rules: &HashSet<PageOrdering>) -> bool {
    update.iter().enumerate().all(|(i, left_page)| {
        !update
            .iter()
            .skip(i + 1)
            .any(|right_page| rules.contains(&(*right_page, *left_page)))
    })
}

fn part1(rules: &HashSet<PageOrdering>, updates: &Vec<Vec<usize>>) -> usize {
    updates
        .iter()
        .filter(|update| update_is_ordered(update, rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(rules: &HashSet<PageOrdering>, updates: &Vec<Vec<usize>>) -> usize {
    updates
        .iter()
        .filter(|update| !update_is_ordered(update, rules))
        .map(|update| {
            let mut update = update.clone();
            update.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else if rules.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
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
    let (rules, updates) = parse(&lines);

    println!("Part 1: {}", part1(&rules, &updates));
    println!("Part 2: {}", part2(&rules, &updates));
}
