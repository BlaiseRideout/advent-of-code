use std::collections::HashMap;
use std::io;

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_char_poss(grid: &Vec<Vec<char>>, needle: char) -> Vec<(usize, usize)> {
    grid.into_iter()
        .enumerate()
        .filter_map(|(start_y, row)| {
            if let Some(start_x) = row.into_iter().position(|c| *c == needle) {
                Some((start_x, start_y))
            } else {
                None
            }
        })
        .collect()
}

fn find_char_pos(grid: &Vec<Vec<char>>, needle: char) -> (usize, usize) {
    find_char_poss(&grid, needle)[0]
}

fn add(lhs: (isize, isize), rhs: (isize, isize)) -> (isize, isize) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

fn char_to_height(c: char) -> isize {
    match c {
        'a'..='z' => Some(c),
        'S' => Some('a'),
        'E' => Some('z'),
        _ => None,
    }
    .expect("Couldn't get height for char") as isize
}

fn path_for_starting_pos(grid: &Vec<Vec<char>>, start_pos: (usize, usize)) -> Option<usize> {
    let height: usize = grid.len();
    let width: usize = grid[0].len();
    let end_pos = find_char_pos(&grid, 'E');
    let mut checked_paths = HashMap::<(usize, usize), usize>::new();
    let mut paths_to_check = [(start_pos, 0)].into_iter().collect::<HashMap<_, _>>();
    while !checked_paths.contains_key(&end_pos) && paths_to_check.len() != 0 {
        let new_paths_to_check = paths_to_check
            .iter()
            .map(|(path_pos, path_steps)| {
                [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .into_iter()
                    .map(|dir| add(dir, (path_pos.0 as isize, path_pos.1 as isize)))
                    .filter_map(|surrounding_pos| {
                        if (0..width as isize).contains(&surrounding_pos.0)
                            && (0..height as isize).contains(&surrounding_pos.1)
                        {
                            let surrounding_pos =
                                (surrounding_pos.0 as usize, surrounding_pos.1 as usize);

                            if checked_paths.contains_key(&surrounding_pos) {
                                None
                            } else {
                                let surrounding_height =
                                    char_to_height(grid[surrounding_pos.1][surrounding_pos.0]);
                                let c_height = char_to_height(grid[path_pos.1][path_pos.0]);
                                if surrounding_height - 1 <= c_height {
                                    Some((surrounding_pos, path_steps + 1))
                                } else {
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>()
            })
            .reduce(|mut ret, m| {
                ret.extend(m);
                ret
            })
            .expect("Couldn't get new paths to check");
        checked_paths.extend(paths_to_check.drain());

        paths_to_check = new_paths_to_check;
    }
    checked_paths.get(&end_pos).copied()
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    let start_pos = find_char_pos(&grid, 'S');
    path_for_starting_pos(grid, start_pos).expect("Couldn't get path from start pos")
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    find_char_poss(&grid, 'a')
        .into_iter()
        .chain(find_char_poss(&grid, 'S'))
        .filter_map(|start_pos| path_for_starting_pos(&grid, start_pos))
        .min()
        .expect("Couldn't get shortest path")
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    let grid = parse(&lines);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
