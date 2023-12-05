use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type StartSeeds = Vec<usize>;
type StartRangeSeeds = Vec<(usize, usize)>;
type Mapping = (usize, usize, usize);
type SectionMap = Vec<Mapping>;

fn parse_maps(lines: &Vec<String>) -> Vec<SectionMap> {
  lines
    .split(String::is_empty)
    .skip(1)
    .map(|section| {
      section
        .iter()
        .skip(1)
        .map(|map_line| -> (usize, usize, usize) {
          map_line
            .split(" ")
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect_tuple()
            .expect("Couldn't parse tuple")
        })
        .collect()
    })
    .collect()
}

fn parse_simple_seeds(lines: &Vec<String>) -> StartSeeds {
  lines
    .split(String::is_empty)
    .take(1)
    .exactly_one()
    .expect("Couldn't get start seed line")
    .iter()
    .take(1)
    .exactly_one()
    .expect("Couldn't get right side of seed line")
    .split(":")
    .skip(1)
    .take(1)
    .exactly_one()
    .expect("Couldn't split start seed list")
    .split_whitespace()
    .map(str::parse::<usize>)
    .filter_map(Result::ok)
    .collect()
}

fn map_seed_to_location(seed: usize, section_maps: &Vec<SectionMap>) -> usize {
  section_maps.iter().fold(seed, |cur_num, section_map| {
    if let Some((dest_range, source_range, _)) = section_map
      .iter()
      .find(|&(_, source_range, len)| (*source_range..*source_range + *len).contains(&cur_num))
    {
      cur_num - source_range + dest_range
    } else {
      cur_num
    }
  })
}

fn map_location_to_seed(location: usize, section_maps: &Vec<SectionMap>) -> usize {
  section_maps
    .iter()
    .rev()
    .fold(location, |cur_num, section_map| {
      if let Some((dest_range, source_range, _)) = section_map
        .iter()
        .find(|&(dest_range, _, len)| (*dest_range..*dest_range + *len).contains(&cur_num))
      {
        cur_num - dest_range + source_range
      } else {
        cur_num
      }
    })
}

fn parse_range_seeds(lines: &Vec<String>) -> StartRangeSeeds {
  parse_simple_seeds(lines).into_iter().tuples().collect()
}

fn part1(start_seeds: &StartSeeds, section_maps: &Vec<SectionMap>) -> usize {
  start_seeds
    .iter()
    .map(|&start_seed| map_seed_to_location(start_seed, section_maps))
    .min()
    .expect("Couldn't get min location")
}

fn part2(start_seeds: &StartRangeSeeds, section_maps: &Vec<SectionMap>) -> usize {
  (0..usize::MAX)
    .find(|&location_num| {
      let seed_for_loc = map_location_to_seed(location_num, section_maps);
      start_seeds
        .iter()
        .any(|(seed_range_start, seed_range_len)| {
          (*seed_range_start..*seed_range_start + *seed_range_len).contains(&seed_for_loc)
        })
    })
    .expect("Couldn't find answer to part 2")
  /*
  section_maps
    .last()
    .expect("Couldn't get location map")
    .into_iter()
    .sorted_by_key(
      |x| x.0, // sort by dest range
    )
    .filter_map(|&(range_start, _, range_len)| {
      (range_start..range_start + range_len).find(|&location_num| {
        let seed_for_loc = map_location_to_seed(location_num, section_maps);
        start_seeds
          .iter()
          .any(|(seed_range_start, seed_range_len)| {
            (*seed_range_start..*seed_range_start + *seed_range_len).contains(&seed_for_loc)
          })
      })
    })
    .take(1)
    .exactly_one()
    .expect("Couldn't find min seed")
    */
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

  let section_maps = parse_maps(&lines);
  let start_seeds = parse_simple_seeds(&lines);
  let range_seeds = parse_range_seeds(&lines);

  println!("Part 1: {}", part1(&start_seeds, &section_maps));
  println!("Part 2: {}", part2(&range_seeds, &section_maps));
}
