use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Move {
    Left,
    Right,
    Forward(usize),
}

fn parse(lines: &Vec<String>) -> (Vec<Vec<char>>, Vec<Move>) {
    static MOVE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+|[L,R]").unwrap());

    let mut it = lines.split(String::is_empty);
    let grid_str = it.next().expect("Couldn't get grid");
    let moves_str = it.next().expect("Couldn't get list of moves");
    assert_eq!(moves_str.len(), 1);
    let moves_str = &moves_str[0];
    let width = grid_str
        .iter()
        .map(|row| row.len())
        .max()
        .expect("Couldn't get width of rows");
    (
        grid_str
            .iter()
            .map(|row| {
                let mut row = row.chars().collect::<Vec<_>>();
                row.resize(width, ' ');
                row
            })
            .collect(),
        MOVE_RE
            .captures_iter(&moves_str)
            .map(|m| match &m[0] {
                "L" => Move::Left,
                "R" => Move::Right,
                _ => Move::Forward(m[0].parse::<usize>().expect("Couldn't parse move as int")),
            })
            .collect(),
    )
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl Direction {
    fn from_isize(v: isize) -> Direction {
        match v {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => todo!("Can't parse {} as Direction", v),
        }
    }
    fn to_vec2(&self) -> (isize, isize) {
        match &self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

fn add(
    p1: (usize, usize),
    p2: (isize, isize),
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    let p3 = (p1.0 as isize + p2.0, p1.1 as isize + p2.1);
    if p3.0 >= 0 && p3.1 >= 0 {
        let p3 = (p3.0 as usize, p3.1 as usize);
        if p3.1 < width && p3.0 < height {
            Some(p3)
        } else {
            None
        }
    } else {
        None
    }
}

fn print_grid(grid: &Vec<Vec<char>>, history: &HashMap<(usize, usize), Direction>) {
    for x in 0..grid[0].len() {
        if x < 10 {
            print!("{}", x);
        }
    }
    println!("");
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if let Some(direction) = history.get(&(y, x)) {
                print!(
                    "{}",
                    match direction {
                        Direction::Left => "<",
                        Direction::Up => "^",
                        Direction::Right => ">",
                        Direction::Down => "V",
                    }
                );
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!("");
    }
    println!("");
}

static DEBUG: bool = false;
fn part1(grid: &Vec<Vec<char>>, moves: &Vec<Move>) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    let mut pos = (
        0usize,
        min(
            grid[0].iter().position(|c| *c == '.'),
            grid[0].iter().position(|c| *c == '#'),
        )
        .expect("Couldn't get start pos"),
    );
    let mut direction = Direction::Right;
    let mut history = [(pos, direction)]
        .into_iter()
        .collect::<HashMap<(usize, usize), Direction>>();
    if DEBUG {
        print_grid(&grid, &history);
    }
    for m in moves {
        //println!("Move: {:?}", m);
        match m {
            Move::Left => direction = Direction::from_isize((direction as isize - 1).rem_euclid(4)),
            Move::Right => {
                direction = Direction::from_isize((direction as isize + 1).rem_euclid(4))
            }
            Move::Forward(count) => {
                for _ in 0..*count {
                    let offset = direction.to_vec2();
                    let mut new_pos = add(pos, offset, width, height);
                    let c = if let Some(new_pos) = new_pos {
                        grid[new_pos.0][new_pos.1]
                    } else {
                        ' '
                    };
                    match c {
                        '.' => {
                            if let Some(new_pos) = new_pos {
                                pos = new_pos
                            }
                        }
                        '#' => break,
                        ' ' => {
                            match direction {
                                Direction::Left => {
                                    new_pos = Some((
                                        pos.0,
                                        if let Some(right_x) = (pos.1..width)
                                            .into_iter()
                                            .position(|x| grid[pos.0][x] == ' ')
                                        {
                                            right_x - 1
                                        } else {
                                            width - 1
                                        },
                                    ));
                                }
                                Direction::Right => {
                                    new_pos = Some((
                                        pos.0,
                                        if let Some(left_x) = (0..pos.1)
                                            .into_iter()
                                            .rev()
                                            .position(|x| grid[pos.0][x] == ' ')
                                        {
                                            left_x + 1
                                        } else {
                                            0
                                        },
                                    ));
                                }
                                Direction::Up => {
                                    new_pos = Some((
                                        if let Some(down_y) = (pos.0..height)
                                            .rev()
                                            .into_iter()
                                            .position(|y| grid[y][pos.1] == ' ')
                                        {
                                            down_y + 1
                                        } else {
                                            height - 1
                                        },
                                        pos.1,
                                    ));
                                }
                                Direction::Down => {
                                    new_pos = Some((
                                        if let Some(up_y) = (0..pos.0)
                                            .rev()
                                            .into_iter()
                                            .position(|y| grid[y][pos.1] == ' ')
                                        {
                                            up_y + 1
                                        } else {
                                            0
                                        },
                                        pos.1,
                                    ));
                                }
                            }
                            if let Some(new_pos) = new_pos {
                                if grid[new_pos.0][new_pos.1] == '.' {
                                    pos = new_pos;
                                }
                            }
                        }
                        _ => {
                            if let Some(new_pos) = new_pos {
                                todo!("Non covered grid tile case: {}", grid[new_pos.0][new_pos.1])
                            }
                        }
                    }
                    history.insert(pos, direction);
                }
            }
        }
        history.insert(pos, direction);
        if DEBUG {
            print_grid(&grid, &history);
        }
    }
    1000 * dbg!(pos.0 + 1) + 4 * dbg!(pos.1 + 1) + dbg!(direction) as usize
}

fn part2(lines: &Vec<String>) -> usize {
    lines.len()
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

    let (grid, moves) = parse(&lines);

    //println!("Parsed: {:?},{:?}", grid, moves);

    println!("Part 1: {}", part1(&grid, &moves));
    println!("Part 2: {}", part2(&lines));
}
