use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

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

fn segment_bounds(
    dims: (usize, usize),
) -> (Vec<RangeInclusive<usize>>, Vec<RangeInclusive<usize>>) {
    let (width, height) = dims;
    let seg_width = width / 4;
    let seg_height = height / 3;
    (
        (0..4)
            .map(|x| (seg_width * x)..=(seg_width * (x + 1) - 1))
            .collect(),
        (0..3)
            .map(|y| (seg_height * y)..=(seg_height * (y + 1) - 1))
            .collect(),
    )
}

fn get_section(pos: (usize, usize), dims: (usize, usize)) -> usize {
    let (x_segs, y_segs) = segment_bounds(dims);
    let y_seg = y_segs
        .into_iter()
        .position(|seg| seg.contains(&pos.0))
        .expect("Couldn't get seg from y");
    let x_seg = x_segs
        .into_iter()
        .position(|seg| seg.contains(&pos.1))
        .expect("Couldn't get seg from x");
    match y_seg {
        0 => 1,
        1 => match x_seg {
            0 => 2,
            1 => 3,
            2 => 4,
            _ => todo!("Couldn't get section from pos {:?}", pos),
        },
        2 => match x_seg {
            2 => 5,
            3 => 6,
            _ => todo!("Couldn't get section from pos {:?}", pos),
        },
        _ => todo!("Couldn't get section from pos {:?}", pos),
    }
}

fn grid_dims(grid: &Vec<Vec<char>>) -> (usize, usize) {
    (grid[0].len(), grid.len())
}

fn wrap_pos_simple_2d(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    direction: Direction,
) -> (usize, usize) {
    let (width, height) = grid_dims(&grid);
    match direction {
        Direction::Left => (
            pos.0,
            if let Some(right_x) = (pos.1..width)
                .into_iter()
                .position(|x| grid[pos.0][x] == ' ')
            {
                right_x - 1
            } else {
                width - 1
            },
        ),
        Direction::Right => (
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
        ),
        Direction::Up => (
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
        ),
        Direction::Down => (
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
        ),
    }
}

fn wrap_pos_cube(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    direction: Direction,
) -> ((usize, usize), Direction) {
    let dims = grid_dims(&grid);
    let (width, height) = dims;
    let (x_segs, y_segs) = segment_bounds(dims);
    let section = get_section(pos, grid_dims(grid));
    match section {
        1 => match direction {
            Direction::Right => ((height - 1 - pos.0, width - 1), Direction::Left),
            Direction::Up => ((height - 1, pos.1), Direction::Up),
            Direction::Left => (
                (*y_segs[1].start(), x_segs[1].start() + pos.0),
                Direction::Down,
            ),
            Direction::Down => todo!(
                "Shouldn't have wrapped going {:?} from {}",
                direction,
                section
            ),
        },
        2 => match direction {
            Direction::Left => (
                (height - 1, pos.0 - y_segs[1].start() + x_segs[3].start()),
                Direction::Up,
            ),
            Direction::Up => (
                (0, x_segs[0].end() - pos.1 + x_segs[2].start()),
                Direction::Down,
            ),
            Direction::Down => (
                (height - 1, x_segs[0].end() - pos.1 + x_segs[2].start()),
                Direction::Up,
            ),
            Direction::Right => todo!(
                "Shouldn't have wrapped going {:?} from {}",
                direction,
                section
            ),
        },
        3 => match direction {
            Direction::Up => (
                (pos.1 - x_segs[1].start(), *x_segs[2].start()),
                Direction::Right,
            ),
            Direction::Down => (
                (
                    x_segs[1].end() - pos.1 + y_segs[2].start(),
                    *x_segs[2].start(),
                ),
                Direction::Right,
            ),
            Direction::Left => todo!(
                "Shouldn't have wrapped going {:?} from {} ({:?})",
                direction,
                section,
                pos
            ),
            Direction::Right => todo!(
                "Shouldn't have wrapped going {:?} from {}",
                direction,
                section
            ),
        },
        4 => match direction {
            Direction::Right => (
                (
                    *y_segs[2].start(),
                    y_segs[1].end() - pos.0 + x_segs[3].start(),
                ),
                Direction::Down,
            ),
            Direction::Down => todo!(
                "Shouldn't have wrapped going {:?} from {}",
                direction,
                section
            ),
            Direction::Left => todo!(
                "Shouldn't have wrapped going {:?} from {}",
                direction,
                section
            ),
            Direction::Up => todo!(
                "Shouldn't have wrapped going {:?} from {}",
                direction,
                section
            ),
        },
        5 => match direction {
            Direction::Left => (
                (
                    *y_segs[1].end(),
                    y_segs[2].end() - pos.0 + x_segs[1].start(),
                ),
                Direction::Up,
            ),
            Direction::Down => ((*y_segs[1].end(), x_segs[2].end() - pos.1), Direction::Up),
            Direction::Up => todo!(
                "Shouldn't have wrapped going {:?} from {}",
                direction,
                section
            ),
            Direction::Right => todo!(
                "Shouldn't have wrapped going {:?} from {}",
                direction,
                section
            ),
        },
        6 => match direction {
            Direction::Right => ((y_segs[2].end() - pos.1, *x_segs[2].end()), Direction::Left),
            Direction::Up => (
                (
                    x_segs[3].end() - pos.1 + y_segs[1].start(),
                    *x_segs[2].end(),
                ),
                Direction::Left,
            ),
            Direction::Down => (
                (x_segs[3].end() - pos.1 + y_segs[1].start(), 0),
                Direction::Right,
            ),
            Direction::Left => todo!(
                "Shouldn't have wrapped going {:?} from {}",
                direction,
                section
            ),
        },
        _ => todo!("Couldn't wrap from section {}", section),
    }
}

static DEBUG: bool = false;

fn walk_grid(
    grid: &Vec<Vec<char>>,
    moves: &Vec<Move>,
    is_cube: bool,
) -> ((usize, usize), Direction) {
    let (width, height) = grid_dims(&grid);
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
        println!("Move: {:?}", m);
        match m {
            Move::Left => direction = Direction::from_isize((direction as isize - 1).rem_euclid(4)),
            Move::Right => {
                direction = Direction::from_isize((direction as isize + 1).rem_euclid(4))
            }
            Move::Forward(count) => {
                for _ in 0..*count {
                    let offset = direction.to_vec2();
                    let new_pos = add(pos, offset, width, height);
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
                            let (new_pos, new_dir) = if is_cube {
                                wrap_pos_cube(&grid, pos, direction)
                            } else {
                                (wrap_pos_simple_2d(&grid, pos, direction), direction)
                            };
                            if grid[new_pos.0][new_pos.1] == '.' {
                                println!(
                                    "Wrapped {:?} {:?} to {:?} {:?}",
                                    pos, direction, new_pos, new_dir
                                );
                                pos = new_pos;
                                direction = new_dir;
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
    (pos, direction)
}

fn part1(grid: &Vec<Vec<char>>, moves: &Vec<Move>) -> usize {
    let (pos, direction) = walk_grid(&grid, moves, false);
    1000 * dbg!(pos.0 + 1) + 4 * dbg!(pos.1 + 1) + dbg!(direction) as usize
}

fn part2(grid: &Vec<Vec<char>>, moves: &Vec<Move>) -> usize {
    let (pos, direction) = walk_grid(&grid, moves, true);
    1000 * dbg!(pos.0 + 1) + 4 * dbg!(pos.1 + 1) + dbg!(direction) as usize
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

    //println!("Part 1: {}", part1(&grid, &moves));
    println!("Part 2: {}", part2(&grid, &moves));
}
