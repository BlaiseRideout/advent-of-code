use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn parse(lines: &Vec<String>) -> HashMap<usize, Vec<usize>> {
    lines
        .iter()
        .filter_map(|line| {
            line.split(": ").collect_tuple().and_then(|(k, v)| {
                Some((
                    k.parse::<usize>().expect("Couldn't get value"),
                    v.split_whitespace()
                        .map(str::parse::<usize>)
                        .filter_map(Result::ok)
                        .collect(),
                ))
            })
        })
        .collect()
}

fn equation_works((result, operands): &(&usize, &Vec<usize>), operand_count: usize) -> bool {
    (0..operand_count.pow(operands.len() as u32 - 1))
        .find(|possibility| {
            (1..operands.len()).fold(
                operands.first().expect("Couldn't get init operand").clone(),
                |acc, ind| {
                    let operand = operands[ind];
                    let op_type =
                        (possibility / (operand_count.pow(ind as u32 - 1))) % operand_count;
                    if op_type == 0 {
                        acc + operand
                    } else if op_type == 1 {
                        acc * operand
                    } else {
                        (acc.to_string() + &operand.to_string())
                            .parse::<usize>()
                            .expect("Couldn't parse concatenation result")
                    }
                },
            ) == **result
        })
        .is_some()
}

fn part1(equations: &HashMap<usize, Vec<usize>>) -> usize {
    equations
        .iter()
        .filter(|equation| equation_works(equation, 2))
        .map(|(result, _)| result)
        .sum()
}

fn part2(equations: &HashMap<usize, Vec<usize>>) -> usize {
    equations
        .iter()
        .filter(|equation| equation_works(equation, 3))
        .map(|(result, _)| result)
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
    let equations = parse(&lines);

    println!("Part 1: {}", part1(&equations));
    println!("Part 2: {}", part2(&equations));
}
