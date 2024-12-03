use once_cell::sync::Lazy;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

static MUL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap());

type MulOp = (usize, usize);

fn parse(lines: &Vec<String>, always_enabled: bool) -> Vec<MulOp> {
    let line = lines.join("");
    MUL_RE
        .captures_iter(line.as_str())
        .fold((true, vec![]), |(enabled, mut mulops), captures| {
            match (
                captures.get(0).expect("Couldn't get match").as_str(),
                enabled || always_enabled,
            ) {
                ("do()", _) => (true, mulops),
                ("don't()", _) => (false, mulops),
                (_, true) => {
                    mulops.push((
                        str::parse::<usize>(
                            captures.get(1).expect("Couldn't get first num").as_str(),
                        )
                        .expect("Couldn't parse num"),
                        str::parse::<usize>(
                            captures.get(2).expect("Couldn't get first num").as_str(),
                        )
                        .expect("Couldn't parse num"),
                    ));
                    (enabled, mulops)
                }
                (_, _) => (enabled, mulops),
            }
        })
        .1
}

fn sum_ops(ops: &Vec<MulOp>) -> usize {
    ops.iter().map(|(x, y)| x * y).sum()
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

    println!("Part 1: {}", sum_ops(&parse(&lines, true)));
    println!("Part 2: {}", sum_ops(&parse(&lines, false)));
}
