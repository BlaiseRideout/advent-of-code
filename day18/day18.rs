use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type Cube = (isize, isize, isize);

fn parse_cubes(lines: &Vec<String>) -> HashSet<Cube> {
    lines
        .iter()
        .filter_map(|line| {
            line.split(",")
                .map(str::trim)
                .map(str::parse::<isize>)
                .filter_map(Result::ok)
                .next_tuple::<Cube>()
        })
        .collect()
}

static ADJACENT_COORDS: Lazy<Vec<Cube>> = Lazy::new(|| {
    (-1..=1)
        .into_iter()
        .map(move |x| {
            (-1..=1)
                .into_iter()
                .map(move |y| {
                    (-1..=1)
                        .into_iter()
                        .filter_map(|z| {
                            if [x, y, z].into_iter().filter(|a| *a == 0).count() == 2 {
                                Some((x, y, z))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
        })
        .flatten()
        .collect()
});

fn add(c1: &Cube, c2: &Cube) -> Cube {
    (c1.0 + c2.0, c1.1 + c2.1, c1.2 + c2.2)
}
fn mul(c1: &Cube, n: isize) -> Cube {
    (c1.0 * n, c1.1 * n, c1.2 * n)
}

fn part1(cubes: &HashSet<Cube>) -> usize {
    println!(
        "ADJACENT_COORDS: ({}){:?}",
        ADJACENT_COORDS.len(),
        *ADJACENT_COORDS
    );

    cubes
        .iter()
        .map(|cube| {
            ADJACENT_COORDS
                .iter()
                .map(|coord| {
                    let adjacent_cube = add(&cube, coord);
                    if cubes.contains(&adjacent_cube) {
                        0
                    } else {
                        1
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

//if cube_state ==

/*
if (x, y, z) == (2, 2, 5) {
    dbg!(ADJACENT_COORDS
        .iter()
        .all(|c| cubes.contains(&add(c, &cube))));
    dbg!(cubes.contains(&(x, y, z)));
}*/
/*
let blocked_directions = (-bound..=bound)
    .into_iter()
    .map(|n| {
        ADJACENT_COORDS
            .iter()
            .positions(|c| cubes.contains(&add(&mul(c, n), &cube)))
            .collect::<HashSet<usize>>()
    })
    .reduce(|mut hs1, hs2| {
        hs1.extend(&hs2);
        hs1
    })
    .expect("Couldn't count blocked directions");
if blocked_directions.len() == ADJACENT_COORDS.len()
    &&
{
    1
} else {
    0
}*/

fn generate_cubes<F: FnMut(Cube) -> bool>(bound: isize, mut condition: F) -> HashSet<Cube> {
    (-bound..=bound)
        .into_iter()
        .filter_map(|x| {
            (-bound..=bound)
                .into_iter()
                .map(|y| {
                    (-bound..=bound)
                        .into_iter()
                        .filter_map(|z| {
                            let cube = (x, y, z);

                            if condition(cube) {
                                Some(cube)
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<Cube>>()
                })
                .reduce(|mut hs1, hs2| {
                    hs1.extend(&hs2);
                    hs1
                })
        })
        .reduce(|mut hs1, hs2| {
            hs1.extend(&hs2);
            hs1
        })
        .expect("Couldn't generate cube set")
}

fn part2(cubes: &HashSet<Cube>) -> usize {
    let total_surface = part1(cubes);

    let bound = cubes
        .iter()
        .map(|cube| [cube.0, cube.1, cube.2])
        .flatten()
        .map(isize::abs)
        .max()
        .expect("Couldn't get max bound");

    let empty_cubes: HashSet<Cube> = generate_cubes(bound, Box::new(|cube| !cubes.contains(&cube)));

    let boundary_cubes: HashSet<Cube> = generate_cubes(bound, |cube| {
        [cube.0, cube.1, cube.2]
            .into_iter()
            .map(isize::abs)
            .filter(|a| *a == bound)
            .count()
            == 3
            && !cubes.contains(&cube)
    });
    dbg!(boundary_cubes.len());

    let start_cube = boundary_cubes
        .into_iter()
        .next()
        .expect("Couldn't get start cube");

    let mut to_check = [start_cube].into_iter().collect::<Vec<Cube>>();
    let mut checked = HashSet::<Cube>::new();
    while !to_check.is_empty() {
        let cube = to_check.pop().expect("Couldn't get cube to check");
        //dbg!(to_check.len());
        //dbg!(checked.len());
        checked.insert(cube);
        ADJACENT_COORDS.iter().for_each(|coord| {
            let adjacent_cube = add(&cube, coord);
            if empty_cubes.contains(&adjacent_cube) && !checked.contains(&adjacent_cube) {
                to_check.push(adjacent_cube);
            }
        });
    }
    dbg!(total_surface);
    dbg!(empty_cubes.len());
    dbg!(checked.len());

    total_surface - (empty_cubes.len() - checked.len()) * 6
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

    let cubes = parse_cubes(&lines);

    //println!("Cubes: {:?}", cubes);

    println!("Part 1: {}", part1(&cubes));
    println!("Part 2: {}", part2(&cubes));
}
