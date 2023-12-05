use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type Point = (usize, usize);
#[derive(Debug)]
struct Num {
  pos: Point,
  len: usize,
  val: usize,
}
#[derive(Debug)]
struct Board {
  symbols: HashMap<Point, char>,
  nums: Vec<Num>,
}
impl Default for Board {
  fn default() -> Board {
    Board {
      symbols: HashMap::new(),
      nums: vec![],
    }
  }
}

fn parse(lines: &Vec<String>) -> Board {
  lines
    .iter()
    .enumerate()
    .map(|(y, line)| {
      line
        .chars()
        .chain(['.']) // Finish numbers on the right
        .enumerate()
        .fold((Board::default(), None), |(board, cur_num), (x, char)| {
          if char.is_digit(10) {
            (
              board,
              Some(cur_num.unwrap_or(String::from("")) + &char.to_string()),
            )
          } else {
            (
              Board {
                symbols: if char != '.' {
                  board.symbols.into_iter().chain([((x, y), char)]).collect()
                } else {
                  board.symbols
                },
                nums: if let Some((num_len, num_val)) =
                  cur_num.map(|n| (n.len(), n.parse::<usize>().expect("Couldn't parse num")))
                {
                  board
                    .nums
                    .into_iter()
                    .chain([Num {
                      pos: (x - num_len, y),
                      val: num_val,
                      len: num_len,
                    }])
                    .collect()
                } else {
                  board.nums
                },
              },
              None,
            )
          }
        })
        .0
    })
    .reduce(|b1, b2| Board {
      symbols: b1.symbols.into_iter().chain(b2.symbols).collect(),
      nums: b1.nums.into_iter().chain(b2.nums).collect(),
    })
    .expect("Couldn't parse board")
}

fn part1(board: &Board) -> usize {
  board
    .nums
    .iter()
    .filter(|num| {
      (max(num.pos.0, 1) - 1..=num.pos.0 + num.len)
        .cartesian_product(max(num.pos.1, 1) - 1..=num.pos.1 + 1)
        .any(|pos| board.symbols.contains_key(&pos))
    })
    .map(|num| num.val)
    .sum()
}

fn part2(board: &Board) -> usize {
  board
    .symbols
    .iter()
    .filter(|(_, &char)| char == '*')
    .map(|(&pos, _)| {
      board
        .nums
        .iter()
        .filter(|num| {
          (max(num.pos.0, 1) - 1..=num.pos.0 + num.len).contains(&pos.0)
            && (max(num.pos.1, 1) - 1..=num.pos.1 + 1).contains(&pos.1)
        })
        .map(|num| num.val)
        .collect::<Vec<_>>()
    })
    .filter(|adjacent_nums| adjacent_nums.len() == 2)
    .map(|adjacent_nums| adjacent_nums.into_iter().product::<usize>())
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

  let board = parse(&lines);

  // WRONG: 323955
  // WRONG: 519922
  // WRONG: 526868
  // WRONG: 528547
  // RIGHT! 521601
  println!("Part 1: {}", part1(&board));
  println!("Part 2: {}", part2(&board));
}
