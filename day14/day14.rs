use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io;

type Point = (isize, isize);

#[derive(PartialEq, Eq, Clone)]
struct Bounds {
    minx: isize,
    maxx: isize,
    miny: isize,
    maxy: isize,
}

#[derive(PartialEq, Eq, Clone)]
struct Grid {
    rocks: HashSet<Point>,
    rock_bounds: Bounds,
    sands: HashSet<Point>,
    sand_bounds: Bounds,
    pt2: bool,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Occupation {
    Rock,
    Sand,
    Air,
}

fn add(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

impl Grid {
    fn parse(lines: &Vec<String>) -> Grid {
        let merge_hashsets =
            |hs1: HashSet<Point>, hs2: HashSet<Point>| hs1.union(&hs2).cloned().collect();
        Self::new(
            lines
                .into_iter()
                .filter_map(|line| {
                    line.split("->")
                        .map(|point| -> Point {
                            let mut pointstrs = point.trim().split(",");
                            (
                                pointstrs
                                    .next()
                                    .expect("Couldn't get x coord")
                                    .parse::<isize>()
                                    .expect("Couldn't parse x coord"),
                                pointstrs
                                    .next()
                                    .expect("Couldn't get y coord")
                                    .parse::<isize>()
                                    .expect("Couldn't parse y coord"),
                            )
                        })
                        .tuple_windows::<(_, _)>()
                        .filter_map(|rock| {
                            (min(rock.0 .0, rock.1 .0)..=max(rock.0 .0, rock.1 .0))
                                .map(|x| {
                                    (min(rock.0 .1, rock.1 .1)..=max(rock.0 .1, rock.1 .1))
                                        .map(|y| (x, y))
                                        .collect::<HashSet<Point>>()
                                })
                                .reduce(merge_hashsets)
                        })
                        .reduce(merge_hashsets)
                })
                .reduce(merge_hashsets)
                .expect("Couldn't get rocks"),
        )
    }

    fn new(rocks: HashSet<Point>) -> Grid {
        let rock_bounds = Bounds {
            minx: rocks
                .iter()
                .map(|rock| rock.0)
                .min()
                .expect("Couldn't get minx"),
            maxx: rocks
                .iter()
                .map(|rock| rock.0)
                .max()
                .expect("Couldn't get maxx"),
            miny: rocks
                .iter()
                .map(|rock| rock.1)
                .min()
                .expect("Couldn't get miny"),
            maxy: rocks
                .iter()
                .map(|rock| rock.1)
                .max()
                .expect("Couldn't get maxy"),
        };
        Grid {
            sand_bounds: rock_bounds.clone(),
            rock_bounds,
            rocks,
            sands: [].into_iter().collect(),
            pt2: false,
        }
    }

    fn set_pt2(&mut self, pt2: bool) {
        self.pt2 = pt2;
    }

    fn position_occupied_by(&self, p: Point) -> Occupation {
        if self.pt2 && p.1 >= self.rock_bounds.maxy + 2 {
            Occupation::Rock
        } else if self.rocks.contains(&p) {
            Occupation::Rock
        } else if self.sands.contains(&p) {
            Occupation::Sand
        } else {
            Occupation::Air
        }
    }

    fn drop_sand(&mut self, start_pos: Point) -> Option<Point> {
        let mut sand_pos: Point = start_pos;
        while sand_pos.1 <= self.rock_bounds.maxy + 2 {
            if let Some(dest) = [(0, 1), (-1, 1), (1, 1)]
                .into_iter()
                .map(|p| add(sand_pos, p))
                .find(|p| self.position_occupied_by(*p) == Occupation::Air)
            {
                sand_pos = dest;
            } else {
                self.sand_bounds.minx = min(self.sand_bounds.minx, sand_pos.0);
                self.sand_bounds.maxx = max(self.sand_bounds.maxx, sand_pos.0);
                self.sand_bounds.miny = min(self.sand_bounds.miny, sand_pos.1);
                self.sand_bounds.maxy = max(self.sand_bounds.maxy, sand_pos.1);
                self.sands.insert(sand_pos);
                return Some(sand_pos);
            }
        }
        None
    }

    fn print(&self) {
        let minx = min(self.rock_bounds.minx, self.sand_bounds.minx);
        let miny = min(self.rock_bounds.miny, self.sand_bounds.miny);
        let maxx = max(self.rock_bounds.maxx, self.sand_bounds.maxx);
        let maxy = max(self.rock_bounds.maxy, self.sand_bounds.maxy);
        println!(
            "{}",
            (0..3)
                .rev()
                .map(|digit| {
                    let power = isize::pow(10, digit);
                    "  ".to_string()
                        + &(minx..=maxx)
                            .map(|x| {
                                if x % 10 == 0 || x == maxx || x == minx {
                                    ((x / power) % 10).to_string()
                                } else {
                                    " ".to_string()
                                }
                            })
                            .join("")
                })
                .join("\n")
        );
        for y in min(miny, 0)..=maxy {
            print!("{} ", y);
            for x in minx..=maxx {
                print!(
                    "{}",
                    match self.position_occupied_by((x, y)) {
                        Occupation::Rock => "#",
                        Occupation::Sand => "o",
                        Occupation::Air => ".",
                    }
                );
            }
            println!("");
        }
    }
}

static DEBUG: bool = false;

fn part1(mut grid: Grid) -> usize {
    if DEBUG {
        grid.print();
    }

    while let Some(settled_pos) = grid.drop_sand((500, 0)) {
        if DEBUG {
            grid.print();
            println!("Sand settled at ({}, {})", settled_pos.0, settled_pos.1);
        }
    }
    grid.sands.len()
}

fn part2(mut grid: Grid) -> usize {
    grid.set_pt2(true);
    if DEBUG {
        grid.print();
    }
    let start_pos: Point = (500, 0);
    while grid.drop_sand(start_pos) != Some(start_pos) {
        if DEBUG {
            grid.print();
        }
    }
    grid.sands.len()
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let grid = Grid::parse(&lines);

    println!("Part 1: {}", part1(grid.clone()));
    println!("Part 2: {}", part2(grid));
}
