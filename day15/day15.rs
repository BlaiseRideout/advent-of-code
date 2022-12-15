use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
    range: isize,
}

impl Sensor {
    fn new(position: Point, beacon: Point) -> Sensor {
        Sensor {
            position,
            beacon,
            range: manhattan(beacon, position),
        }
    }
}

fn parse(lines: &Vec<String>) -> Vec<Sensor> {
    static SENSOR_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)")
            .unwrap()
    });
    lines
        .into_iter()
        .filter_map(|line| -> Option<Sensor> {
            let parsed_line = SENSOR_RE
                .captures_iter(line)
                .next()
                .expect("Couldn't match regex");
            Some(Sensor::new(
                Point {
                    x: parsed_line[1].parse::<isize>().ok()?,
                    y: parsed_line[2].parse::<isize>().ok()?,
                },
                Point {
                    x: parsed_line[3].parse::<isize>().ok()?,
                    y: parsed_line[4].parse::<isize>().ok()?,
                },
            ))
        })
        .collect()
}

fn manhattan(p1: Point, p2: Point) -> isize {
    isize::abs(p2.x - p1.x) + isize::abs(p2.y - p1.y)
}

fn part1(sensors: &Vec<Sensor>, y: isize) -> usize {
    let beacon_positions: HashSet<isize> = sensors
        .iter()
        .filter(|sensor| sensor.beacon.y == y)
        .map(|sensor| sensor.beacon.x)
        .collect();
    let sensor_positions: HashSet<isize> = sensors
        .iter()
        .filter(|sensor| sensor.position.y == y)
        .map(|sensor| sensor.position.x)
        .collect();

    sensors
        .into_iter()
        .map(|sensor| {
            let horizontal_range_at_y = sensor.range - isize::abs(sensor.position.y - y);
            ((sensor.position.x - horizontal_range_at_y)
                ..=(sensor.position.x + horizontal_range_at_y))
                .collect::<HashSet<_>>()
        })
        .fold(HashSet::<isize>::new(), |mut s1, s2| {
            s1.extend(&s2);
            s1
        })
        .difference(&beacon_positions)
        .copied()
        .collect::<HashSet<_>>()
        .difference(&sensor_positions)
        .copied()
        .collect::<HashSet<_>>()
        .len()
}

fn part2(lines: &Vec<String>) -> usize {
    lines.len()
}

fn main() {
    if env::args().count() != 3 {
        return println!(
            "Usage: {} [path/to/input_file] [y of interest]",
            env::args().next().expect("Couldn't get executable name")
        );
    }
    let input_name: String = env::args().skip(1).next().expect("First argument");
    let y_of_interest: isize = env::args()
        .skip(2)
        .next()
        .expect("Second argument")
        .parse::<isize>()
        .expect("Couldn't parse second argument as int");
    let f = File::open(input_name).expect("Couldn't open input file");
    let lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let parsed = parse(&lines);

    println!("Part 1: {}", part1(&parsed, y_of_interest));
    println!("Part 2: {}", part2(&lines));
}
