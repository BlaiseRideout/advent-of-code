use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Tile {
  Ash,
  Rocks,
}
type Parsed = Vec<Vec<Vec<Tile>>>;
fn parse(lines: &Vec<String>) -> Parsed {
  lines
    .split(String::is_empty)
    .map(|lines| {
      lines
        .iter()
        .map(|line| {
          line
            .chars()
            .map(|c| match c {
              '#' => Tile::Rocks,
              _ => Tile::Ash,
            })
            .collect()
        })
        .collect()
    })
    .collect()
}

trait Diff {
  fn diff(&self, s: &Self) -> usize;
}
impl Diff for Tile {
  fn diff(&self, t2: &Self) -> usize {
    if t2 != self {
      1
    } else {
      0
    }
  }
}
impl<T> Diff for Vec<T>
where
  T: Diff,
{
  fn diff(&self, t2: &Self) -> usize {
    self
      .iter()
      .interleave_shortest(t2.iter())
      .tuples()
      .map(|(lhs, rhs)| lhs.diff(rhs))
      .sum()
  }
}

fn find_splits<T: Eq + Debug + Diff>(vec: &Vec<T>) -> HashMap<usize, usize> {
  (1..vec.len())
    .map(|split_ind| {
      (
        split_ind,
        vec[0..split_ind]
          .iter()
          .rev()
          .interleave_shortest(vec[split_ind..].iter())
          .tuples()
          .map(|(a, b)| a.diff(b))
          .sum(),
      )
    })
    .collect()
}

fn find_split<T: Eq + Debug + Diff>(vec: &Vec<T>, delta: usize) -> HashSet<usize> {
  find_splits(&vec)
    .iter()
    .filter(|(_, count)| **count == delta)
    .map(|(split_ind, _)| split_ind)
    .copied()
    .collect::<HashSet<_>>()
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
  T: Copy,
{
  assert!(!v.is_empty());
  (0..v[0].len())
    .map(|i| v.iter().map(|inner| inner[i]).collect::<Vec<T>>())
    .collect()
}

fn solve(grids: &Parsed, delta: usize) -> usize {
  grids
    .iter()
    .map(|grid| -> usize {
      let horizontal_splits = find_split(grid, delta);
      let vertical_splits = find_split(&transpose(&grid), delta);
      (if let Ok(vertical_split) = vertical_splits.iter().take(1).exactly_one() {
        *vertical_split
      } else {
        0
      }) + (if let Ok(horizontal_split) = horizontal_splits.iter().take(1).exactly_one() {
        horizontal_split * 100
      } else {
        0
      })
    })
    .sum()
}

fn part1(grid: &Parsed) -> usize {
  solve(grid, 0)
}
fn part2(grid: &Parsed) -> usize {
  solve(grid, 1)
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
  // WRONG: 39114
  // MAYBE? 3700
  println!("Part 2: {}", part2(&parsed));
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;

  fn sampledata() -> Vec<String> {
    vec![
      "#.##..##.".to_string(),
      "..#.##.#.".to_string(),
      "##......#".to_string(),
      "##......#".to_string(),
      "..#.##.#.".to_string(),
      "..##..##.".to_string(),
      "#.#.##.#.".to_string(),
      "".to_string(),
      "#...##..#".to_string(),
      "#....#..#".to_string(),
      "..##..###".to_string(),
      "#####.##.".to_string(),
      "#####.##.".to_string(),
      "..##..###".to_string(),
      "#....#..#".to_string(),
    ]
  }

  #[rstest]
  #[case(sampledata(), 405)]
  fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: usize) {
    assert_eq!(expected, part1(&parse(&input)));
  }

  #[rstest]
  #[case(sampledata(), 405)]
  fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: usize) {
    assert_eq!(expected, part2(&parse(&input)));
  }
}
