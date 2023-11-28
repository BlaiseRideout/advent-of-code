use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
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
            (sensor.position.x - horizontal_range_at_y)
                ..=(sensor.position.x + horizontal_range_at_y)
        })
        .fold(HashSet::<isize>::new(), |mut s1, s2| {
            s1.extend(s2);
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

fn part2_ranges(sensors: &Vec<Sensor>, max_coord: isize) -> isize {
    let minx: isize = 0;
    let maxx: isize = max_coord;

    let beacon_point = (0..=max_coord)
        .find_map(|y| {
            type Range = (isize, isize);
            let mut x_ranges = sensors
                .iter()
                .filter_map(|sensor| {
                    let horizontal_range_at_y = sensor.range - isize::abs(sensor.position.y - y);
                    if horizontal_range_at_y > 0 {
                        Some((
                            min(max(sensor.position.x - horizontal_range_at_y, minx), maxx),
                            max(min(sensor.position.x + horizontal_range_at_y, maxx), minx),
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            x_ranges.sort_by(|r1, r2| isize::cmp(&r1.0, &r2.0));
            let reduced_range = x_ranges
                .into_iter()
                .reduce(|r1, r2| {
                    if r1.1 < r2.0 {
                        r1
                    } else {
                        (min(r1.0, r2.0), max(r1.1, r2.1))
                    }
                })
                .expect("Couldn't get reduced range");

            if reduced_range.1 != maxx {
                let x = reduced_range.1 + 1;
                Some(Point { x, y })
            } else if reduced_range.0 != minx {
                let x = reduced_range.0 - 1;
                Some(Point { x, y })
            } else {
                None
            }
        })
        .expect("Couldn't find beacon point");

    println!(
        "Possible sensor location: {},{}",
        beacon_point.x, beacon_point.y
    );
    beacon_point.x * 4000000 + beacon_point.y
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return println!("Usage: {} [path/to/input_file] [pt1 y] [pt2 y]", args[0]);
    }
    let input_name: &String = &args[1];
    let pt1_y: isize = args[2]
        .parse::<isize>()
        .expect("Couldn't parse second argument as int");
    let pt2_y: isize = args[3]
        .parse::<isize>()
        .expect("Couldn't parse third argument as int");
    let f = File::open(input_name).expect("Couldn't open input file");
    let lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let parsed = parse(&lines);

    println!("Part 1: {}", part1(&parsed, pt1_y));
    println!("Part 2: {}", part2_ranges(&parsed, pt2_y));
}
