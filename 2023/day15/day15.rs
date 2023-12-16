use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn parse(lines: &Vec<String>) -> Vec<&str> {
  lines
    .first()
    .expect("couldn't get first line")
    .split(",")
    .collect()
}

fn hash(s: &str) -> usize {
  s.chars().fold(0, |cur, c| ((cur + c as usize) * 17) % 256)
}
fn part1(parsed: &Vec<&str>) -> usize {
  parsed.iter().map(|line| hash(line)).sum()
}

fn part2(parsed: &Vec<&str>) -> usize {
  let mut lenses = HashMap::<usize, Vec<(&str, usize)>>::new();
  parsed.iter().for_each(|line| {
    let is_eq = line.find('=').is_some();
    let (label, focal_length) = line
      .split(['-', '='])
      .tuples()
      .take(1)
      .exactly_one()
      .expect("Couldn't split line");
    let hash = hash(label);
    if !lenses.contains_key(&hash) {
      lenses.insert(hash, vec![]);
    }
    let lens_box = lenses.get_mut(&hash).unwrap();
    if is_eq {
      let focal_length = focal_length.parse::<usize>().unwrap();
      if let Some(replacement_lens) = lens_box.iter_mut().find(|(name, _)| *name == label) {
        replacement_lens.1 = focal_length;
      } else {
        lens_box.push((label, focal_length));
      }
    } else if let Some((remove_lens_ind, _)) = lens_box
      .iter_mut()
      .find_position(|(name, _)| *name == label)
    {
      lens_box.remove(remove_lens_ind);
    }
  });
  lenses
    .iter()
    .map(|(box_num, lenses)| {
      lenses
        .iter()
        .enumerate()
        .map(|(slot, focal_length)| (box_num + 1) * (slot + 1) * focal_length.1)
        .map(|x| x)
        .sum::<usize>()
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
  let parsed = parse(&lines);

  println!("Part 1: {}", part1(&parsed));
  println!("Part 2: {}", part2(&parsed));
}
