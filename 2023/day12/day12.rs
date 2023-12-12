use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use indicatif::ProgressIterator;
use itertools::Itertools;
use pariter::{scope, IteratorExt as _};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Spring {
  Damaged,
  Operational,
  Unknown,
  ConsumedDamaged,
}
type Row = Vec<Spring>;
type Counts = Vec<usize>;
type Parsed = Vec<(Row, Counts)>;

fn parse(lines: &Vec<String>) -> Parsed {
  lines
    .iter()
    .map(|line| {
      let (row, counts) = line
        .split_whitespace()
        .tuples()
        .take(1)
        .exactly_one()
        .expect("Couldn't split row");
      (
        row
          .chars()
          .map(|c| match c {
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            _ => Spring::Unknown,
          })
          .collect(),
        counts
          .split(",")
          .map(str::parse::<usize>)
          .filter_map(Result::ok)
          .collect(),
      )
    })
    .collect()
}

fn solve_row(row: Row, counts: Counts) -> usize {
  let mut to_solve = vec![(row.clone(), counts)];
  let mut solved: Vec<Row> = vec![];
  while !to_solve.is_empty() {
    let (row, mut counts) = to_solve.pop().unwrap();
    if counts.is_empty() && !row.contains(&&Spring::Damaged) {
      solved.push(row);
      continue;
    }
    let last_count = counts.pop();
    if last_count.is_none() {
      continue;
    }
    let last_count = last_count.unwrap();
    let min_consumed = row.iter().find_position(|&x| *x == Spring::ConsumedDamaged);
    to_solve.extend(
      row
        .windows(last_count)
        .enumerate()
        .rev()
        .skip_while(|(i, window)| {
          if let Some((min_consumed, _)) = min_consumed {
            min_consumed <= *i + last_count
              || (*i + last_count == min_consumed && *window.last().unwrap() != Spring::Operational)
          } else {
            false
          }
        })
        .scan(None, |least_damaged_ind, (i, window)| {
          if let (Some((operational_ind, _)), Some(least_damaged_ind)) = (
            window.iter().find_position(|&x| *x == Spring::Operational),
            &least_damaged_ind,
          ) {
            if i + operational_ind < *least_damaged_ind {
              return None;
            }
          }
          if let Some((damaged_ind, _)) = window.iter().find_position(|&x| *x == Spring::Damaged) {
            *least_damaged_ind = Some(i + damaged_ind);
          }
          return Some((i, window));
        })
        .filter(|(_, window)| {
          window
            .iter()
            .all(|&spring| spring == Spring::Damaged || spring == Spring::Unknown)
        })
        .map(|(i, _)| {
          let mut new_row = row.clone();
          (i..i + last_count).for_each(|i| {
            new_row[i] = Spring::ConsumedDamaged;
          });
          (i + last_count..new_row.len()).for_each(|i| {
            if new_row[i] == Spring::Unknown {
              new_row[i] = Spring::Operational;
            }
          });
          (new_row, counts.clone())
        }),
    );
  }
  solved.len()
}

fn part1(springs: &Parsed) -> usize {
  springs
    .iter()
    .map(|(row, counts)| solve_row(row.clone(), counts.clone()))
    //.map(|combinations| dbg!(combinations))
    .sum()
}

fn part2(springs: &Parsed) -> usize {
  scope(|scope| {
    springs
      .iter()
      .map(|(row, counts)| {
        (
          (0..5)
            .map(|_| row.iter())
            .flatten()
            .to_owned()
            .copied()
            .collect(),
          (0..5)
            .map(|_| counts.iter())
            .flatten()
            .to_owned()
            .copied()
            .collect(),
        )
      })
      .progress()
      .parallel_map_scoped(scope, |(row, counts)| solve_row(row, counts))
      .sum()
  })
  .unwrap()
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

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;

  fn sampledata() -> Vec<String> {
    vec![
      "???.### 1,1,3".to_string(),
      ".??..??...?##. 1,1,3".to_string(),
      "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
      "????.#...#... 4,1,1".to_string(),
      "????.######..#####. 1,6,5".to_string(),
      "?###???????? 3,2,1".to_string(),
    ]
  }

  #[rstest]
  #[case(sampledata(), 21)]
  #[case(vec!["???.### 1,1,3".to_string()], 1)]
  #[case(vec![".??..??...?##. 1,1,3".to_string()], 4)]
  #[case(vec!["?#?#?#?#?#?#?#? 1,3,1,6".to_string()], 1)]
  #[case(vec!["????.#...#... 4,1,1".to_string()], 1)]
  #[case(vec!["????.######..#####. 1,6,5".to_string()], 4)]
  #[case(vec!["?###???????? 3,2,1".to_string()], 10)]
  #[case(vec!["?###????? 3,2,1".to_string()], 1)]
  fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: usize) {
    assert_eq!(expected, part1(&parse(&input)));
  }

  /*
  #[rstest]
  #[case(sampledata(), 1)]
  fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: usize) {
    assert_eq!(expected, part2(&parse(&input)));
  }
   */
}
