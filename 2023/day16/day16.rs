use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::mem::swap;
use std::ops::Add;

use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Debug)]
enum Tile {
  ForwardMirror,
  BackMirror,
  HSplitter,
  VSplitter,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Vec2 {
  x: isize,
  y: isize,
}

#[derive(Eq, PartialEq, Debug)]
struct Board {
  tiles: HashMap<Vec2, Tile>,
  width: isize,
  height: isize,
}

fn parse(lines: &Vec<String>) -> Board {
  Board {
    tiles: lines
      .iter()
      .enumerate()
      .map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| {
          Some((
            Vec2 {
              x: x as isize,
              y: y as isize,
            },
            match c {
              '/' => Some(Tile::ForwardMirror),
              '\\' => Some(Tile::BackMirror),
              '|' => Some(Tile::VSplitter),
              '-' => Some(Tile::HSplitter),
              _ => None,
            }?,
          ))
        })
      })
      .flatten()
      .collect(),
    height: lines.len() as isize,
    width: lines.first().unwrap().len() as isize,
  }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Beam {
  //i: usize,
  p: Vec2,
  v: Vec2,
}

impl Add for Vec2 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

fn display(board: &Board, energized: &HashSet<Vec2>) {
  println!(
    "{}",
    (0..board.width)
      .map(|y| (0..board.height)
        .map(|x| {
          let p = Vec2 { x: x, y: y };
          if energized.contains(&p) {
            '#'
          } else {
            '.'
          }
        })
        .collect::<String>())
      .join("\n")
  );
}

static RIGHT: Vec2 = Vec2 { x: 1, y: 0 };
static DOWN: Vec2 = Vec2 { x: 0, y: 1 };

fn energized_from_start(board: &Board, start: Beam) -> usize {
  let mut energized = HashSet::<Beam>::new();
  let mut beams = vec![start];
  while !beams.is_empty() {
    //dbg!(&beams);
    let mut beam = beams.pop().unwrap();
    beam.p = beam.p + beam.v;
    if !(0..board.width).contains(&beam.p.x) || !(0..board.height).contains(&beam.p.y) {
      continue;
    }
    if energized.contains(&beam) {
      continue;
    }
    energized.insert(beam);
    if let Some(tile) = board.tiles.get(&beam.p) {
      match tile {
        Tile::BackMirror => {
          // \
          swap(&mut beam.v.x, &mut beam.v.y);
        }
        Tile::ForwardMirror => {
          // /
          // (1, 0) => (0, -1)
          // (0, 1) => (-1, 0)
          swap(&mut beam.v.x, &mut beam.v.y);
          beam.v.x *= -1;
          beam.v.y *= -1;
        }
        Tile::HSplitter => {
          //  -
          if beam.v.x == 0 {
            beam.v = RIGHT;
            //beam.i *= 2;
            beams.push(beam.clone());
            //beam.i += 1;
            beam.v.x *= -1;
          }
        }
        Tile::VSplitter => {
          //  |
          if beam.v.y == 0 {
            beam.v = DOWN;
            beams.push(beam.clone());
            beam.v.y *= -1;
          }
        }
      }
    }
    beams.push(beam);
  }
  /*
  let all_energized = energized
    .into_values()
    .reduce(|a, b| a.intersection(&b).copied().collect())
    .expect("Couldn't get all energized");
    */

  let energized = energized
    .into_iter()
    .map(|beam| beam.p)
    .collect::<HashSet<_>>();
  //display(&board, &energized);
  //all_energized.len()
  //dbg!(&energized);
  energized.len()
}

fn part1(board: &Board) -> usize {
  energized_from_start(
    board,
    Beam {
      //i: 0,
      p: Vec2 { x: -1, y: 0 },
      v: RIGHT,
    },
  )
}

fn part2(board: &Board) -> usize {
  (0..max(board.width, board.height))
    .map(|pos_magnitude| {
      let mut energized = vec![];
      let mut insert_beam = |beam: Beam| {
        energized.push(energized_from_start(board, beam));
      };

      let swap_dir = |beam: &mut Beam| {
        beam.p.x *= -1;
        beam.p.y *= -1;
      };
      let mut beam = Beam {
        p: Vec2 {
          x: -1,
          y: pos_magnitude,
        },
        v: RIGHT,
      };
      insert_beam(beam);
      swap(&mut beam.p.x, &mut beam.p.y);
      swap(&mut beam.v.x, &mut beam.v.y);
      insert_beam(beam);
      beam.p.x = board.width;
      beam.p.y = pos_magnitude;
      swap(&mut beam.v.x, &mut beam.v.y);
      swap_dir(&mut beam);
      insert_beam(beam);
      beam.p.x = pos_magnitude;
      beam.p.y = board.height;
      swap(&mut beam.v.x, &mut beam.v.y);
      insert_beam(beam);

      energized
    })
    .flatten()
    .into_iter()
    .max()
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
  let board = parse(&lines);

  println!("Part 1: {}", part1(&board));
  println!("Part 2: {}", part2(&board));
}
