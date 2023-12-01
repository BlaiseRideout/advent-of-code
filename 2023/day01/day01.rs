use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

fn digits_to_value(num1: char, num2: char) -> usize {
    [num1, num2]
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn part1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| {
            let num1 = line.chars().find(|c| *c >= '0' && *c <= '9').unwrap_or('0');
            let num2 = line
                .chars()
                .rfind(|c| *c >= '0' && *c <= '9')
                .unwrap_or('0');
            digits_to_value(num1, num2)
        })
        .sum()
}

static DIGIT_TO_STR: Lazy<HashMap<&str, u8>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("one", 1);
    m.insert("two", 2);
    m.insert("three", 3);
    m.insert("four", 4);
    m.insert("five", 5);
    m.insert("six", 6);
    m.insert("seven", 7);
    m.insert("eight", 8);
    m.insert("nine", 9);
    m.insert("zero", 0);
    m
});

fn parse_digit(s: &str) -> char {
    (if let Some(v) = DIGIT_TO_STR.get(s) {
        *v
    } else {
        s.parse::<u8>().expect("Couldn't parse number")
    } + '0' as u8) as char
}

fn part2(lines: &Vec<String>) -> usize {
    static FORWARD_NUM_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            DIGIT_TO_STR
                .keys()
                .map(|k| *k)
                .chain(["[0-9]"].into_iter())
                .intersperse("|")
                .collect::<String>()
                .as_str(),
        )
        .expect("Couldn't parse regex")
    });
    static BACKWARD_NUM_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            DIGIT_TO_STR
                .keys()
                .map(|k| k.chars().rev().collect::<String>())
                .chain([String::from("[0-9]")].into_iter())
                .intersperse(String::from("|"))
                .collect::<String>()
                .as_str(),
        )
        .expect("Couldn't parse regex")
    });

    lines
        .iter()
        .map(|line| {
            let num1 = FORWARD_NUM_RE
                .find(line)
                .map(|s| parse_digit(s.as_str()))
                .expect("Couldn't find first digit");
            let num2 = BACKWARD_NUM_RE
                .find(line.chars().rev().collect::<String>().as_str())
                .map(|s| parse_digit(s.as_str().chars().rev().collect::<String>().as_str()))
                .expect("Couldn't find reverse match");
            digits_to_value(num1, num2)
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
