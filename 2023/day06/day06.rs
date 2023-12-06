use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn parse(lines: &Vec<String>) -> Vec<(usize, usize)> {
  let (times, distances) = lines
    .iter()
    .map(|line| {
      line
        .split(":")
        .skip(1)
        .exactly_one()
        .expect("Couldn't split line")
        .split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
    })
    .tuples()
    .exactly_one()
    .expect("Couldn't find times and distances");
  times.interleave(distances).tuples().collect()
}

fn part1(parsed: &Vec<(usize, usize)>) -> usize {
  parsed
    .iter()
    .map(|(race_time, distance_record)| {
      (0..=*race_time)
        .map(|button_time| button_time * (race_time - button_time))
        .filter(|distance| distance > distance_record)
        .count()
    })
    .product()
}

fn parse2(lines: &Vec<String>) -> (usize, usize) {
  lines
    .iter()
    .map(|line| {
      line
        .split(":")
        .skip(1)
        .exactly_one()
        .expect("Couldn't split line")
        .replace(" ", "")
        .parse::<usize>()
        .expect("Couldn't parse number")
    })
    .tuples()
    .exactly_one()
    .expect("Couldn't find times and distances")
}

fn part2(parsed: &(usize, usize)) -> usize {
  let &(race_time, distance_record) = parsed;
  (0..=race_time)
    .rev()
    .find(|button_time| (button_time * (race_time - button_time)) > distance_record)
    .expect("Couldn't find end")
    - (0..=race_time)
      .find(|button_time| (button_time * (race_time - button_time)) > distance_record)
      .expect("Couldn't find start")
    + 1
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
  let parsed2 = parse2(&lines);

  println!("Part 1: {}", part1(&parsed));
  println!("Part 2: {}", part2(&parsed2));
}
