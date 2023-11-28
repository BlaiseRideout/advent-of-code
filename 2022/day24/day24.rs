use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
    COUNT = 4,
}

impl Direction {
    fn from_isize(v: isize) -> Direction {
        match v {
            0 => Direction::Right,
            1 => Direction::Left,
            2 => Direction::Up,
            3 => Direction::Down,
            4 => Direction::COUNT,
            _ => todo!("Can't parse {} as Direction", v),
        }
    }
    fn from_char(c: char) -> Direction {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            '^' => Direction::Up,
            _ => todo!("Direction char not implemented: {}", c),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Direction::Right => '>',
            Direction::Left => '<',
            Direction::Down => 'v',
            Direction::Up => '^',
            _ => todo!("Direction char not implemented: {:?}", self),
        }
    }

    fn to_vec2(&self) -> (isize, isize) {
        match &self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            _ => panic!("Shouldn't be converting Direction::COUNT to vec2"),
        }
    }
}

type Point = (isize, isize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Blizzard {
    dir: Direction,
    pos: Point,
}

fn add(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

fn print_grid(blizzards: &Vec<Blizzard>, wh: Point) {
    let (width, height) = wh;
    for y in 0..height {
        print!("#");
        for x in 0..width {
            let pos_blizzards = blizzards
                .iter()
                .filter(|blizzard| blizzard.pos == (x, y))
                .collect::<Vec<_>>();
            if pos_blizzards.len() == 0 {
                print!(".");
            } else if pos_blizzards.len() == 1 {
                print!("{}", pos_blizzards[0].dir.to_char());
            } else if pos_blizzards.len() < 10 {
                print!("{}", pos_blizzards.len());
            } else {
                print!("+");
            }
        }
        println!("#");
    }
    println!("");
}

fn parse(lines: &Vec<String>) -> (Vec<Blizzard>, Point) {
    let height = lines.len() - 2; // Remove the border walls
    let width = lines[0].len() - 2;
    (
        lines
            .iter()
            .skip(1)
            .enumerate()
            .filter_map(|(y, line)| {
                if y < height {
                    Some(line.chars().skip(1).enumerate().filter_map(move |(x, c)| {
                        if x < width && c != '.' {
                            Some(Blizzard {
                                pos: (x as isize, y as isize),
                                dir: Direction::from_char(c),
                            })
                        } else {
                            None
                        }
                    }))
                } else {
                    None
                }
            })
            .flatten()
            .collect(),
        (width as isize, height as isize),
    )
}

static DEBUG: bool = false;

fn path_through_blizzard(
    mut blizzards: Vec<Blizzard>,
    wh: Point,
    destinations: Vec<Point>,
) -> usize {
    let (width, height) = wh;
    let entrance: Point = (0, -1);
    let exit: Point = (width - 1, height);

    let mut i_destination = 0;
    let mut destination = destinations[i_destination];
    let mut paths: HashSet<Point> = [entrance].into_iter().collect();

    let direction_vecs = (0..Direction::COUNT as isize)
        .into_iter()
        .map(|dir| Direction::from_isize(dir).to_vec2())
        .collect::<Vec<_>>();

    for minute in 1..=usize::MAX {
        if DEBUG {
            println!("");
            print_grid(&blizzards, wh);
        }

        let blizzard_poss = blizzards
            .iter_mut()
            .map(|blizzard| {
                blizzard.pos = add(blizzard.pos, blizzard.dir.to_vec2());
                blizzard.pos.0 = blizzard.pos.0.rem_euclid(width);
                blizzard.pos.1 = blizzard.pos.1.rem_euclid(height);
                blizzard.pos
            })
            .collect::<HashSet<_>>();

        let new_paths = paths
            .iter()
            .map(|pos| {
                direction_vecs
                    .iter()
                    .map(|dir| add(*pos, *dir))
                    .filter(|pos| {
                        (pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height)
                            || *pos == entrance
                            || *pos == exit
                    })
                    .filter(|pos| !blizzard_poss.contains(&pos))
                    .filter(|pos| !paths.contains(&pos))
            })
            .flatten()
            .collect::<HashSet<_>>();

        paths.extend(new_paths);

        // Remove paths that have a blizzard on them
        paths.retain(|pos| !blizzard_poss.contains(&pos));

        if DEBUG {
            println!("Minute {}: {} paths", minute, paths.len());
        }

        if paths.contains(&destination) {
            paths = [destination].into_iter().collect();
            i_destination += 1;
            if i_destination == destinations.len() {
                return minute;
            }
            destination = destinations[i_destination];
        }
    }
    0
}

fn part1(blizzards: &Vec<Blizzard>, wh: Point) -> usize {
    let (width, height) = wh;
    let exit: Point = (width - 1, height);

    path_through_blizzard(blizzards.clone(), wh, vec![exit])
}

fn part2(blizzards: &Vec<Blizzard>, wh: Point) -> usize {
    let (width, height) = wh;
    let entrance: Point = (0, -1);
    let exit: Point = (width - 1, height);

    path_through_blizzard(blizzards.clone(), wh, vec![exit, entrance, exit])
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

    let (parsed, wh) = parse(&lines);

    println!("Part 1: {}", part1(&parsed, wh));
    println!("Part 2: {}", part2(&parsed, wh));
}
