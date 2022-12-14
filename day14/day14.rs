use itertools::Itertools;
use once_cell::sync::Lazy;
use std::cell::Cell;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io;
use std::ops::RangeInclusive;

type Point = (isize, isize);
type Rock = (RangeInclusive<isize>, RangeInclusive<isize>);

#[derive(PartialEq, Eq, Clone)]
struct Grid {
    rocks: Vec<Rock>,
    minx: isize,
    maxx: isize,
    miny: isize,
    maxy: isize,
    sands: HashSet<Point>,
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
        Self::new(
            lines
                .into_iter()
                .map(|line| -> Vec<Rock> {
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
                        .map(|rock| {
                            (
                                min(rock.0 .0, rock.1 .0)..=max(rock.0 .0, rock.1 .0),
                                min(rock.0 .1, rock.1 .1)..=max(rock.0 .1, rock.1 .1),
                            )
                        })
                        .collect()
                })
                .reduce(|mut rs1, mut rs2| {
                    rs1.append(&mut rs2);
                    rs1
                })
                .expect("Couldn't get rocks"),
        )
    }
    fn new(rocks: Vec<Rock>) -> Grid {
        Grid {
            minx: *rocks
                .iter()
                .map(|rock| rock.0.start())
                .min()
                .expect("Couldn't get minx"),
            maxx: *rocks
                .iter()
                .map(|rock| rock.0.end())
                .max()
                .expect("Couldn't get maxx"),
            miny: *rocks
                .iter()
                .map(|rock| rock.1.start())
                .min()
                .expect("Couldn't get miny"),
            maxy: *rocks
                .iter()
                .map(|rock| rock.1.end())
                .max()
                .expect("Couldn't get maxy"),
            rocks,
            sands: [].into_iter().collect(),
        }
    }

    fn add_ground(&mut self) {
        self.maxy += 2;
        self.rocks
            .push((isize::MIN..=isize::MAX, self.maxy..=self.maxy));
    }

    fn position_occupied_by(&self, p: Point) -> Occupation {
        if self
            .rocks
            .iter()
            .any(|rock| rock.0.contains(&p.0) && rock.1.contains(&p.1))
        {
            Occupation::Rock
        } else if self.sands.contains(&p) {
            Occupation::Sand
        } else {
            Occupation::Air
        }
    }

    fn drop_sand(&mut self, start_pos: Point) -> Option<Point> {
        let mut sand_pos: Point = start_pos;
        while sand_pos.1 <= self.maxy {
            if let Some(dest) = [(0, 1), (-1, 1), (1, 1)]
                .into_iter()
                .map(|p| {
                    let p = add(sand_pos, p);
                    (p, self.position_occupied_by(p))
                })
                .find(|(_, occupation)| *occupation == Occupation::Air)
            {
                sand_pos = dest.0;
            } else {
                self.sands.insert(sand_pos);
                return Some(sand_pos);
            }
        }
        None
    }

    fn print(&self) {
        println!("{} .. {}", self.minx, self.maxx);
        for y in min(self.miny, 0)..=self.maxy {
            print!("{} ", y);
            for x in self.minx..=self.maxx {
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

static DEBUG: bool = true;

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
    grid.add_ground();
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
