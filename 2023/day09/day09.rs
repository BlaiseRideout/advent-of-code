use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type Sequence = Vec<isize>;

fn parse(lines: &Vec<String>) -> Vec<Sequence> {
  lines
    .iter()
    .map(|line| {
      line
        .split_whitespace()
        .map(str::parse::<isize>)
        .filter_map(Result::ok)
        .collect()
    })
    .collect()
}

fn extrapolate_sequence(sequence: &Sequence) -> isize {
  let mut diffs = vec![sequence.clone()];
  loop {
    let next_diffs = diffs
      .last()
      .expect("Couldn't get top of stack")
      .iter()
      .tuple_windows()
      .map(|(x, y)| y - x)
      .collect_vec();
    if next_diffs.iter().all(|x| *x == 0) {
      break;
    } else {
      diffs.push(next_diffs);
    }
  }
  diffs.iter().rev().fold(0, |x, seq| {
    seq.last().expect("Couldn't get last element of sequence") + x
  })
}

fn extrapolate_sequence_rev(sequence: &Sequence) -> isize {
  let mut diffs = vec![sequence.clone()];
  loop {
    let next_diffs = diffs
      .last()
      .expect("Couldn't get top of stack")
      .iter()
      .tuple_windows()
      .map(|(x, y)| y - x)
      .collect_vec();
    if next_diffs.iter().all(|x| *x == 0) {
      break;
    } else {
      diffs.push(next_diffs);
    }
  }
  diffs.iter().rev().fold(0, |x, seq| {
    seq.first().expect("Couldn't get last element of sequence") - x
  })
}

fn part1(sequences: &Vec<Sequence>) -> isize {
  sequences
    .iter()
    .map(|sequence| extrapolate_sequence(sequence))
    .sum()
}

fn part2(sequences: &Vec<Sequence>) -> isize {
  sequences
    .iter()
    .map(|sequence| extrapolate_sequence_rev(sequence))
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
