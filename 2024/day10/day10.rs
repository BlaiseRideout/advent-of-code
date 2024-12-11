use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use indicatif::ProgressBar;

fn parse(lines: &Vec<String>) -> Vec<usize> {
    lines
        .first()
        .expect("Couldn't get frist line")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("coul/dn't parse num"))
        .collect()
}

fn do_stuff(stones: &Vec<usize>, iters: usize) -> usize {
    let progress = ProgressBar::new(iters as u64);
    let a = (0..iters)
        .fold(stones.clone(), |stones, _| {
            //dbg!(&stones);
            progress.inc(1);
            stones
                .iter()
                .map(|stone| {
                    let stone_str = stone.to_string();
                    if *stone == 0 {
                        [1, usize::max_value()]
                    } else if stone_str.len() % 2 == 0 {
                        [
                            stone_str[0..(stone_str.len() / 2)]
                                .parse::<usize>()
                                .expect("Couldn't parse left half"),
                            stone_str[(stone_str.len() / 2)..]
                                .parse::<usize>()
                                .expect("Couldn't parse right half"),
                        ]
                    } else {
                        [*stone * 2024, usize::max_value()]
                    }
                })
                .flatten()
                .filter(|x| *x != usize::max_value())
                .collect()
        })
        .len();
    progress.finish();
    a
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

    println!("Part 1: {}", do_stuff(&parsed, 25));
    println!("Part 2: {}", do_stuff(&parsed, 75));
}
