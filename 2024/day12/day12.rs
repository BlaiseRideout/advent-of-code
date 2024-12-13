use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

fn parse(lines: &Vec<String>) -> Vec<Machine> {
    let extract_vec = |line: &String| {
        NUM_RE
            .captures_iter(line)
            .take(2)
            .map(|capture| {
                capture
                    .get(0)
                    .expect("Couldn't get capture str")
                    .as_str()
                    .parse::<usize>()
                    .expect("Couldn't parse capture num")
            })
            .collect_tuple()
    };
    lines
        .split(String::is_empty)
        .map(|lines| Machine {
            a: extract_vec(&lines[0]).expect("Couldn't parse button A"),
            b: extract_vec(&lines[1]).expect("Couldn't parse button B"),
            prize: extract_vec(&lines[2]).expect("Couldn't parse prize"),
        })
        .collect()
}

fn solve_machine(machine: &Machine, search_range: RangeInclusive<usize>) -> Option<(usize, usize)> {
    search_range
        .clone()
        .cartesian_product(search_range)
        .map(|(a, b)| {
            (
                (a, b),
                (
                    a * machine.a.0 + b * machine.b.0,
                    a * machine.a.1 + b * machine.b.1,
                ),
            )
        })
        .find(|(_, pos)| *pos == machine.prize)
        .map(|(ab, _)| ab)
}

fn part1(machines: &Vec<Machine>) -> usize {
    machines
        .iter()
        .filter_map(|machine| solve_machine(machine, 0..=100))
        .map(|(a, b)| 3 * a + b)
        .sum()
}

fn part2(mut machines: Vec<Machine>) -> usize {
    let p2_factor = 10000000000000;
    machines.iter_mut().for_each(|machine| {
        machine.prize.0 += p2_factor;
        machine.prize.1 += p2_factor;
    });

    machines
        .iter()
        .map(|machine| {
            (
                machine,
                [
                    [machine.a.0, machine.b.0]
                        .iter()
                        .map(|button| machine.prize.0 / button)
                        .collect_vec(),
                    [machine.a.1, machine.b.1]
                        .iter()
                        .map(|button| machine.prize.1 / button)
                        .collect_vec(),
                ]
                .into_iter()
                .flatten()
                .collect_vec(),
            )
        })
        .filter_map(|(machine, ratios)| {
            solve_machine(
                machine,
                *ratios.iter().min().unwrap()..=*ratios.iter().max().unwrap(),
            )
        })
        //.map(|(a, b)| dbg!((a, b)))
        .map(|(a, b)| 3 * a + b)
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
    let machines = parse(&lines);

    println!("Part 1: {}", part1(&machines));
    println!("Part 2: {}", part2(machines));
}
