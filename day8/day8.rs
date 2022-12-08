use std::{collections::HashSet, io};

fn parse_grid(lines: &Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>())
                .filter_map(Result::ok)
                .collect()
        })
        .collect()
}

fn part1(grid: &Vec<Vec<i32>>) -> usize {
    let mut visible_trees = HashSet::<(usize, usize)>::new();

    // From top
    for (y, row) in grid.iter().enumerate() {
        let mut highest: i32 = -1;

        // From top-left
        for (x, &tree) in row.iter().enumerate() {
            if tree > highest {
                visible_trees.insert((x, y));
                highest = tree;
            }
        }

        // From top-right
        highest = -1;
        for (x, &tree) in row.iter().enumerate().rev() {
            if tree > highest {
                visible_trees.insert((x, y));
                highest = tree;
            }
        }
    }

    // From left
    for x in 0..grid[0].len() {
        let mut highest: i32 = -1;
        // From left-top
        for (y, row) in grid.iter().enumerate() {
            let tree: i32 = row[x];
            if tree > highest {
                visible_trees.insert((x, y));
                highest = tree;
            }
        }

        highest = -1;
        // From left-bottom
        for (y, row) in grid.iter().enumerate().rev() {
            let tree: i32 = row[x];
            if tree > highest {
                visible_trees.insert((x, y));
                highest = tree;
            }
        }
    }

    // From right
    for x in grid[0].len()..0 {
        let mut highest: i32 = -1;
        // From right-bottom
        for (y, row) in grid.iter().enumerate().rev() {
            let tree: i32 = row[x];
            if tree > highest {
                visible_trees.insert((x, y));
                highest = tree;
            }
        }

        highest = -1;
        // From right-top
        for (y, row) in grid.iter().enumerate() {
            let tree: i32 = row[x];
            if tree > highest {
                visible_trees.insert((x, y));
                highest = tree;
            }
        }
    }

    // From bottom
    for (y, row) in grid.iter().enumerate().rev() {
        let mut highest: i32 = 0;
        // From bottom-right
        for (x, &tree) in row.iter().enumerate().rev() {
            if tree > highest {
                visible_trees.insert((x, y));
                highest = tree;
            }
        }

        highest = -1;
        // From bottom-left
        for (x, &tree) in row.iter().enumerate() {
            if tree > highest {
                visible_trees.insert((x, y));
                highest = tree;
            }
        }
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, &tree) in row.iter().enumerate() {
            if visible_trees.contains(&(x, y)) {
                print!("{}", tree);
            } else {
                print!("X");
            }
        }
        println!("");
    }

    visible_trees.len()
}

fn calc_scenic_score(grid: &Vec<Vec<i32>>, tree_x: usize, tree_y: usize) -> usize {
    let tree = grid[tree_y][tree_x];
    let mut score: usize = 1;
    // To right
    {
        let mut score_right = 0;
        for x in (tree_x + 1)..grid[0].len() {
            if grid[tree_y][x] < tree {
                score_right += 1;
            } else {
                score_right += 1;
                break;
            }
        }
        //println!("Score right: {}", score_right);
        score *= score_right;
    }
    // To left
    {
        let mut score_left = 0;
        for x in (0..tree_x).rev() {
            if grid[tree_y][x] < tree {
                score_left += 1;
            } else {
                score_left += 1;
                break;
            }
        }
        score *= score_left;
        //println!("Score left: {}", score_left);
    }
    // To top
    {
        let mut score_top = 0;
        for y in (0..tree_y).rev() {
            if grid[y][tree_x] < tree {
                score_top += 1;
            } else {
                score_top += 1;
                break;
            }
        }
        score *= score_top;
        //println!("Score top: {}", score_top);
    }
    // To bottom
    {
        let mut score_bottom = 0;
        for y in (tree_y + 1)..grid.len() {
            if grid[y][tree_x] < tree {
                score_bottom += 1;
            } else {
                score_bottom += 1;
                break;
            }
        }
        score *= score_bottom;
        //println!("Score bottom: {}", score_bottom);
    }
    score
}

fn part2(grid: &Vec<Vec<i32>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tree)| calc_scenic_score(&grid, x, y))
                .max()
                .expect("Couldn't find max for row")
        })
        .max()
        .expect("Couldn't find max for column")
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let grid = parse_grid(&lines);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
