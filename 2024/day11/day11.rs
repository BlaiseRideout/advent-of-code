use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

type Grid = Vec<Vec<char>>;
fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn print_grid(grid: &Grid) {
    println!("{}", grid.iter().map(|row| row.iter().join("")).join("\n"));
}

fn adjacent_nodes((x, y): &(isize, isize)) -> Vec<(isize, isize)> {
    (-1..=1isize)
        .cartesian_product(-1..=1isize)
        .filter(|(deltax, deltay)| deltax.abs() - deltay.abs() != 0)
        .map(move |(deltax, deltay)| (x + deltax, y + deltay))
        .collect()
}
fn plot_perimeter_nodes(plot: &HashSet<(isize, isize)>) -> Vec<(isize, isize)> {
    plot.iter()
        .map(|pos| {
            adjacent_nodes(pos)
                .into_iter()
                .filter(|v| !plot.contains(v))
        })
        .flatten()
        .collect()
}

type Plots = Vec<HashSet<(isize, isize)>>;

fn plot_terrain(grid: &Grid) -> Plots {
    let mut plots: Vec<HashSet<(isize, isize)>> = vec![];
    let (width, height) = (grid.first().unwrap().len(), grid.len());
    //let mut visited_notes = HashSet::<(isize, isize)>::new();
    while let Some(((x, y), plotval)) = grid.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .enumerate()
            .map(|(x, cell)| ((x as isize, y as isize), *cell))
            .find(|((x, y), _)| !plots.iter().any(|plot| plot.contains(&(*x, *y))))
        //.find(|((x, y), _)| !visited_notes.contains(&(*x, *y)))
    }) {
        //dbg!(&plotval);
        plots.push(HashSet::<(isize, isize)>::new());
        let plot = plots.last_mut().expect("Couldn't get new plot");
        plot.insert((x, y));
        //visited_notes.insert((x, y));
        loop {
            let additional_plot_cells = plot
                .iter()
                .map(|pos| {
                    adjacent_nodes(pos)
                        .into_iter()
                        .filter(|v| !plot.contains(v))
                        .filter(|(testx, testy)| *testx >= 0 && *testy >= 0)
                        .map(|(testx, testy)| (testx as usize, testy as usize))
                        .filter(|(testx, testy)| *testx < width && *testy < height)
                        .filter(|(testx, testy)| grid[*testy][*testx] == plotval)
                        .map(|(testx, testy)| (testx as isize, testy as isize))
                })
                .flatten()
                .collect_vec();
            if additional_plot_cells.len() == 0 {
                break;
            }
            plot.extend(additional_plot_cells.iter());
            //visited_notes.extend(additional_plot_cells.iter());
        }
    }
    plots
}

fn part1(plots: &Plots) -> usize {
    plots
        .iter()
        .map(|plot| {
            let area = plot.len();
            let perimeter = plot_perimeter_nodes(&plot);
            area * perimeter.len()
        })
        .sum()
}

fn part2(plots: &Plots) -> usize {
    plots
        .iter()
        .map(|plot| {
            let perimeter = plot_perimeter_nodes(plot).into_iter().fold(
                HashMap::<_, usize>::new(),
                |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    acc
                },
            );
            //.collect::<HashSet<_>>();
            let mut visited_counts = HashMap::<(isize, isize), usize>::new();
            //let mut visited_dirs = HashSet::<((isize, isize), (isize, isize))>::new();
            let mut sides = 0;
            while let Some((cur_node, _)) = perimeter
                .iter()
                .find(|(node, count)| visited_counts.get(node) != Some(*count))
            {
                *visited_counts.entry(*cur_node).or_insert(0) += 1;
                //let cur_count = visited_counts.get(cur_node).unwrap();
                sides += 1;
                if let Some((deltax, deltay)) = adjacent_nodes(cur_node)
                    .iter()
                    //.filter(|dir| !visited_dirs.contains(&(*cur_node, *dir)))
                    .filter(|node| perimeter.contains_key(node))
                    .filter(|node| visited_counts.get(node) != perimeter.get(node))
                    //.filter(|node| visited_counts.get(node) != Some(cur_count))
                    .take(1)
                    .exactly_one()
                    .ok()
                    .map(|next_node| (next_node.0 - cur_node.0, next_node.1 - cur_node.1))
                {
                    //visited_dirs.insert((*cur_node, (deltax, deltay)));
                    let new_dir_nodes = (1..)
                        .map(|i| (cur_node.0 + deltax * i, cur_node.1 + deltay * i))
                        .take_while(|test_dir_node| {
                            //if !visited_dirs.contains(&(test_dir_node, (deltax, deltay))) {
                            perimeter.contains_key(&test_dir_node)
                                && visited_counts.get(&test_dir_node)
                                    != perimeter.get(&test_dir_node)
                            //visited_counts.get(&test_dir_node) != Some(cur_count)
                            /*
                            {
                                Some(test_dir_node)
                            } else {
                                None
                            }
                             */
                        })
                        .chain(
                            (1..)
                                .map(|i| (cur_node.0 + deltax * -i, cur_node.1 + deltay * -i))
                                .take_while(|test_dir_node| {
                                    //if !visited_dirs.contains(&(test_dir_node, (deltax, deltay))) {
                                    /*
                                    {
                                        Some(test_dir_node)
                                    } else {
                                        None
                                    }
                                     */
                                    perimeter.contains_key(&test_dir_node)
                                        && visited_counts.get(&test_dir_node)
                                            != perimeter.get(&test_dir_node)
                                    //visited_counts.get(&test_dir_node) != Some(cur_count)
                                }),
                        )
                        .collect_vec();
                    //dbg!(&new_dir_nodes);
                    new_dir_nodes.into_iter().for_each(|k| {
                        *visited_counts.entry(k).or_insert(0) += 1;
                        //visited_dirs.insert((k, (deltax, deltay)));
                    });
                }
            }
            sides
        })
        .zip(plots.iter().map(|plot| plot.len()))
        .map(|(sides, area)| {
            //println!("{} * {} = {}", sides, area, sides * area);
            (sides, area)
        })
        .map(|(sides, area)| sides * area)
        .sum()
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
    let grid = parse(&lines);
    let plots = plot_terrain(&grid);

    print_grid(&grid);
    println!("Part 1: {}", part1(&plots));
    println!("Part 2: {}", part2(&plots));
}
