use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type Game = (HashSet<usize>, HashSet<usize>);

fn parse(lines: &Vec<String>) -> Vec<Game> {
  lines
    .iter()
    .map(|line| {
      line
        .split(':')
        .skip(1)
        .map(|card| {
          card
            .split("|")
            .map(|section| {
              section
                .split_whitespace()
                .map(|x| x.parse::<usize>().expect("Couldn't parse num"))
                .collect::<HashSet<_>>()
            })
            .tuples()
            .take(1)
            .exactly_one()
            .expect("Couldn't split card sections")
        })
        .take(1)
        .exactly_one()
        .expect("Couldn't parse line")
    })
    .collect()
}

fn score_games(games: &Vec<Game>) -> Vec<usize> {
  games
    .iter()
    .map(|(winning, drawn)| drawn.intersection(&winning).count())
    .collect()
}

fn part1(games: &Vec<Game>) -> usize {
  score_games(games)
    .iter()
    .map(|&num_winning| {
      if num_winning == 0 {
        0
      } else {
        2usize.pow(max(num_winning as u32, 1) - 1)
      }
    })
    .sum::<usize>()
}

fn part2(games: &Vec<Game>) -> usize {
  let scored = score_games(&games);
  let mut games_to_score: Vec<usize> = (0..games.len()).collect();
  let mut i = 0usize;
  while games_to_score.len() > 0 {
    let game = games_to_score
      .pop()
      .expect("Couldn't get top game to score");
    games_to_score.extend(game + 1..=(game + scored[game]));
    i = i + 1;
  }
  i
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
