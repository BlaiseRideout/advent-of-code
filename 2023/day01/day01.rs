use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

fn digits_to_value(nums: Vec<char>) -> usize {
    let digits = if nums.len() < 1 {
        ['0', '0']
    } else {
        [*nums.first().unwrap(), *nums.last().unwrap()]
    };
    digits.iter().join("").parse::<usize>().unwrap()
}

fn part1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| {
            let nums = line
                .chars()
                .filter(|c| *c >= '0' && *c <= '9')
                .collect::<Vec<_>>();
            digits_to_value(nums)
        })
        .sum()
}

fn parse_digit(s: &str) -> char {
    (match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        m => m.parse::<u8>().expect("Couldn't parse number"),
    } + '0' as u8) as char
}

fn part2(lines: &Vec<String>) -> usize {
    static FORWARD_NUM_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine|zero")
            .expect("Couldn't parse regex")
    });
    static BACKWARD_NUM_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"[0-9]|orez|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno")
            .expect("Couldn't parse regex")
    });

    lines
        .iter()
        .map(|line| {
            let num2 = BACKWARD_NUM_RE
                .find(line.chars().rev().collect::<String>().as_str())
                .map(|s| parse_digit(s.as_str().chars().rev().collect::<String>().as_str()))
                .expect("Couldn't find reverse match");
            let num1 = FORWARD_NUM_RE
                .find(line)
                .map(|s| parse_digit(s.as_str()))
                .expect("Couldn't find first digit");
            digits_to_value(vec![num1, num2])
        })
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

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
