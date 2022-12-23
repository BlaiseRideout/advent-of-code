use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type Point = (isize, isize);

fn parse(lines: &Vec<String>) -> HashSet<Point> {
    lines
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .positions(|c| c == '#')
                .map(move |x| (x as isize, y as isize))
        })
        .flatten()
        .collect()
}

fn add(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North = 0,
    South = 1,
    West = 2,
    East = 3,
    COUNT = 4,
}
impl Direction {
    fn from_isize(v: isize) -> Direction {
        match v {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            4 => Direction::COUNT,
            _ => todo!("Can't parse {} as Direction", v),
        }
    }
    fn to_vec2(&self) -> Point {
        match &self {
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::COUNT => todo!("Can't get vec2 from COUNT"),
        }
    }
    fn fields_to_check(&self) -> [Point; 3] {
        match self {
            Direction::West => [(-1, 0), (-1, -1), (-1, 1)],
            Direction::East => [(1, 0), (1, -1), (1, 1)],
            Direction::North => [(0, -1), (-1, -1), (1, -1)],
            Direction::South => [(0, 1), (-1, 1), (1, 1)],
            Direction::COUNT => todo!("Can't get fileds to check from COUNT"),
        }
    }
}

fn elf_bounds(elves: &HashSet<Point>) -> (isize, isize, isize, isize) {
    (
        elves
            .iter()
            .map(|elf| elf.0)
            .min()
            .expect("Couldn't get lower x bound"),
        elves
            .iter()
            .map(|elf| elf.1)
            .min()
            .expect("Couldn't get lower y bound"),
        elves
            .iter()
            .map(|elf| elf.0)
            .max()
            .expect("Couldn't get lower x bound"),
        elves
            .iter()
            .map(|elf| elf.1)
            .max()
            .expect("Couldn't get lower y bound"),
    )
}

fn print_grid(elves: &HashSet<Point>) {
    let (min_x, min_y, max_x, max_y) = elf_bounds(&elves);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

static DEBUG: bool = false;

// Returns the number of empty spaces in the elf bounds, unless rounds is None or high enough to
// reach the equilibrium state, in which case returns the round number where it's reached
fn simulate_rounds(mut elves: HashSet<Point>, rounds: Option<usize>) -> usize {
    let mut first_direction = Direction::North;
    if DEBUG {
        println!("== Initial State ==");
        print_grid(&elves);
    }
    for round in 1..=rounds.unwrap_or(usize::MAX) {
        if DEBUG {
            println!("first direction: {:?}", first_direction);
        }

        let direction_check_order = (0..Direction::COUNT as isize)
            .into_iter()
            .map(|d_i| {
                Direction::from_isize((first_direction as isize + d_i) % Direction::COUNT as isize)
            })
            .collect::<Vec<_>>();

        let proposed_directions = elves
            .iter()
            .filter_map(|elf| -> Option<(Point, Point)> {
                let possible_directions = direction_check_order
                    .iter()
                    .filter(|direction| {
                        direction
                            .fields_to_check()
                            .into_iter()
                            .all(|field| !elves.contains(&add(*elf, field)))
                    })
                    .collect::<Vec<_>>();

                if let (true, Some(possible_direction)) = (
                    possible_directions.len() != direction_check_order.len(),
                    possible_directions.iter().next(),
                ) {
                    Some((add(*elf, possible_direction.to_vec2()), *elf))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if proposed_directions.len() == 0 {
            return round;
        }
        let proposed_direction_counts = proposed_directions.iter().fold(
            HashMap::<(isize, isize), usize>::new(),
            |mut counts, (new_field, _)| {
                counts.insert(
                    *new_field,
                    *counts.get(new_field).unwrap_or(&0usize) + 1usize,
                );
                counts
            },
        );
        if DEBUG {
            println!("Proposed directions: {:?}", proposed_direction_counts);
        }
        proposed_directions
            .into_iter()
            .for_each(|(new_field, old_field)| {
                if *proposed_direction_counts
                    .get(&new_field)
                    .expect("Couldn't get proposed direction count")
                    == 1
                {
                    elves.remove(&old_field);
                    elves.insert(new_field);
                }
            });
        first_direction =
            Direction::from_isize((first_direction as isize + 1) % Direction::COUNT as isize);
        if DEBUG {
            println!("== end of Round {} ==", round);
            print_grid(&elves);
        }
    }
    let (min_x, min_y, max_x, max_y) = elf_bounds(&elves);
    (min_x..=max_x)
        .map(|x| {
            (min_y..=max_y)
                .filter(|y| !elves.contains(&(x, *y)))
                .count()
        })
        .sum()
}

fn part1(elves: &HashSet<Point>) -> usize {
    simulate_rounds(elves.clone(), Some(10))
}

fn part2(elves: &HashSet<Point>) -> usize {
    simulate_rounds(elves.clone(), None)
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

    let elves = parse(&lines);
    //println!("Elves: {:?}", elves);

    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));
}
