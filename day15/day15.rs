use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use threadpool::ThreadPool;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Sensor {
    position: Point,
    range: isize,
}

impl Sensor {
    fn new(position: Point, beacon: Point) -> Sensor {
        Sensor {
            position,
            //beacon,
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
    /*
    let beacon_positions: HashSet<isize> = sensors
        .iter()
        .filter(|sensor| sensor.beacon.y == y)
        .map(|sensor| sensor.beacon.x)
        .collect();*/
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
        //.difference(&beacon_positions)
        //.copied()
        //.collect::<HashSet<_>>()
        .difference(&sensor_positions)
        .copied()
        .collect::<HashSet<_>>()
        .len()
}

fn part2_iterative(sensors: &Vec<Sensor>, max_coord: isize) -> () {
    let pool = ThreadPool::new(24);
    for x in 0..=max_coord {
        let sensors_copy = sensors
            .iter()
            .filter(|sensor| isize::abs(sensor.position.x - x) <= sensor.range)
            .copied()
            .collect::<Vec<Sensor>>();
        pool.execute(move || {
            println!("Working on x = {}", x);
            for y in 0..=max_coord {
                let p = Point { x, y };
                if !sensors_copy
                    .iter()
                    .filter(|sensor| isize::abs(sensor.position.y - p.y) <= sensor.range)
                    .any(|sensor| manhattan(sensor.position, p) <= sensor.range)
                {
                    println!(
                        "Possible sensor location: {},{} ({})",
                        p.x,
                        p.y,
                        p.x * 4000000 + p.y
                    );
                    panic!();
                }
            }
        });
    }
    pool.join();
}

fn part2_sets(sensors: &Vec<Sensor>, max_coord: isize) -> () {
    let minx: isize = 0;
    let maxx: isize = max_coord;

    let pool = ThreadPool::new(24);
    for y in 0..=max_coord {
        let sensors_copy = sensors.clone();
        let y_copy = y;
        pool.execute(move || {
            println!("Working on y = {}", y_copy);
            let xs = sensors_copy
                .iter()
                .map(|sensor| {
                    let horizontal_range_at_y =
                        sensor.range - isize::abs(sensor.position.y - y_copy);

                    (minx..(sensor.position.x - horizontal_range_at_y))
                        .chain((sensor.position.x + horizontal_range_at_y + 1)..=maxx)
                        .collect::<HashSet<_>>()
                })
                .reduce(|s1, s2| s1.intersection(&s2).copied().collect::<HashSet<_>>())
                .expect("Couldn't produce intersection");
            if let Some(distress_pos) = xs.iter().next() {
                println!(
                    "Possible sensor location: {},{} ({})",
                    distress_pos,
                    y,
                    distress_pos * 4000000 + y_copy
                );
                panic!();
            }
        });
    }
    pool.join();
}

fn part2_new(sensors: &Vec<Sensor>, max_coord: isize) {
    let minx: isize = 0;
    let maxx: isize = max_coord;

    let border_points = sensors
        .into_iter()
        .map(|sensor| {
            ((-sensor.range - 1)..=(sensor.range + 1))
                .map(|y_offset| {
                    let y = sensor.position.y + y_offset;
                    let horizontal_range_at_y = sensor.range - isize::abs(y_offset);
                    if horizontal_range_at_y == -1 {
                        [Point {
                            x: sensor.position.x,
                            y,
                        }]
                        .iter()
                        .copied()
                        .collect::<Vec<_>>()
                    } else {
                        [
                            Point {
                                x: sensor.position.x - horizontal_range_at_y - 1,
                                y,
                            },
                            Point {
                                x: sensor.position.x + horizontal_range_at_y + 1,
                                y,
                            },
                        ]
                        .iter()
                        .copied()
                        .collect::<Vec<_>>()
                    }
                })
                .reduce(|mut hs1, hs2| {
                    hs1.extend(&hs2);
                    hs1
                })
                .expect("Couldn't get border points for sensor")
        })
        .reduce(|mut hs1, hs2| {
            hs1.extend(&hs2);
            hs1
        })
        .expect("Couldn't reduce all border points");

    println!("Generated border points");

    for border_point in border_points
        .iter()
        .filter(|p| (minx..=maxx).contains(&p.x) && (minx..=maxx).contains(&p.y))
    {
        if !sensors
            .iter()
            .any(|sensor| manhattan(sensor.position, *border_point) <= sensor.range)
        {
            println!(
                "Possible sensor location: {},{} ({})",
                border_point.x,
                border_point.y,
                border_point.x * 4000000 + border_point.y
            );
            break;
        }
    }
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

    //println!("Part 1: {}", part1(&parsed, y_of_interest));

    println!("Part 2:");
    //part2_sets(&parsed, y_of_interest);
    //part2_iterative(&parsed, y_of_interest);
    part2_new(&parsed, y_of_interest);
}
