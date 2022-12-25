use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use once_cell::sync::Lazy;

static MAX_PLACE: usize = 25;

static PLACE_VALUES: Lazy<Vec<isize>> = Lazy::new(|| {
    (0..=MAX_PLACE)
        .into_iter()
        .map(|place| 5isize.pow(place as u32))
        .collect()
});

static PLACE_MAXES: Lazy<Vec<isize>> = Lazy::new(|| {
    (0..=MAX_PLACE)
        .into_iter()
        .map(|place| (0..=place).map(|i| 5isize.pow(i as u32) * 2).sum())
        .collect()
});

fn parse(lines: &Vec<String>) -> Vec<isize> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .rev()
                .enumerate()
                .fold(0isize, |acc, (place, c)| {
                    acc + match c {
                        '-' => -1isize,
                        '=' => -2isize,
                        '0'..='2' => c as isize - '0' as isize,
                        _ => panic!("Couldn't parse character {}", c),
                    } * PLACE_VALUES[place]
                })
        })
        .collect()
}

fn to_snafu(num: isize) -> String {
    let num_len = PLACE_MAXES
        .iter()
        .position(|place_max| num <= *place_max)
        .expect("Couldn't get size of number")
        + 1;

    static DIGITS: [&str; 5] = ["=", "-", "0", "1", "2"];

    (0..num_len)
        .into_iter()
        .rev()
        .fold(("".to_string(), 0isize), |(mut s, mut acc), place| {
            let remaining = num - acc;

            let digit = if place > 0 {
                (-2..=2)
                    .find(|digit| {
                        ((digit * PLACE_VALUES[place] - PLACE_MAXES[place - 1])
                            ..=(digit * PLACE_VALUES[place] + PLACE_MAXES[place - 1]))
                            .contains(&remaining)
                    })
                    .expect("Couldn't get digit for place")
            } else {
                remaining
            };
            debug_assert!(digit >= -2 && digit <= 2);
            acc += digit * PLACE_VALUES[place];
            s += DIGITS[(digit + 2) as usize];
            (s, acc)
        })
        .0
}

fn part1(parsed: &Vec<isize>) -> String {
    let total_fuel = parsed.iter().sum::<isize>();
    to_snafu(total_fuel)
}

static DEBUG: bool = false;

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
    if DEBUG {
        parsed.iter().for_each(|num| {
            println!("{} -> {}", num, to_snafu(*num));
        });
        println!("{:?}", parsed);
    }

    println!("Part 1: {}", part1(&parsed));
}
