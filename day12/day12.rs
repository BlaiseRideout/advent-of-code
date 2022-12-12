use std::collections::HashMap;
use std::io;

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_char_pos(grid: &Vec<Vec<char>>, needle: char) -> (usize, usize) {
    grid.into_iter()
        .enumerate()
        .find_map(|(start_y, row)| {
            if let Some(start_x) = row.into_iter().position(|c| *c == needle) {
                Some((start_x, start_y))
            } else {
                None
            }
        })
        .expect("Couldn't get char pos")
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

fn add(lhs: (isize, isize), rhs: (isize, isize)) -> (isize, isize) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

fn char_to_height(c: char) -> isize {
    match c {
        'a'..='z' => c as isize - 'a' as isize,
        'S' => 0,
        'E' => 'z' as isize - 'a' as isize,
        _ => 0,
    }
}

fn path_for_starting_pos(grid: &Vec<Vec<char>>, start_pos: (usize, usize)) -> Option<usize> {
    let height: usize = grid.len();
    let width: usize = grid[0].len();
    let end_pos = find_char_pos(&grid, 'E');
    let mut steps_to_field = HashMap::<(usize, usize), usize>::new();
    steps_to_field.insert(start_pos, 0);
    while !steps_to_field.contains_key(&end_pos) {
        let next_steps_to_field = steps_to_field
            .iter()
            .map(|(path_pos, path_steps)| {
                let c_height = char_to_height(grid[path_pos.1][path_pos.0]);

                [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .into_iter()
                    .map(|dir| add(dir, (path_pos.0 as isize, path_pos.1 as isize)))
                    .filter_map(|surrounding_pos| {
                        if (0..width as isize).contains(&surrounding_pos.0)
                            && (0..height as isize).contains(&surrounding_pos.1)
                        {
                            let surrounding_pos =
                                (surrounding_pos.0 as usize, surrounding_pos.1 as usize);

                            if steps_to_field.contains_key(&surrounding_pos) {
                                None
                            } else {
                                let surrounding_height =
                                    char_to_height(grid[surrounding_pos.1][surrounding_pos.0]);
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
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let new_paths_added = next_steps_to_field
            .into_iter()
            .filter_map(|c| {
                if c.len() == 0 {
                    None
                } else {
                    Some(c.into_iter().for_each(|(pos, path_steps)| {
                        if let Some(prev_path_steps) = steps_to_field.get_mut(&pos) {
                            //println!("{:?} = {}", pos, *prev_path_steps);
                            if path_steps < *prev_path_steps {
                                *prev_path_steps = path_steps;
                            }
                        } else {
                            steps_to_field.insert(pos, path_steps);
                        }
                    }))
                }
            })
            .count();
        if new_paths_added == 0 {
            println!("Couldn't get path for start_pos: {:?}", start_pos);
            return None;
        }
    }
    Some(
        *steps_to_field
            .get(&end_pos)
            .expect("Couldn't get end steps"),
    )
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    let start_pos = find_char_pos(&grid, 'S');
    path_for_starting_pos(grid, start_pos).expect("Couldn't get len from start pos")
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    find_char_poss(&grid, 'a')
        .into_iter()
        .chain(find_char_poss(&grid, 'S'))
        .filter_map(|start_pos| path_for_starting_pos(&grid, start_pos))
        .min()
        .expect("Couldn't get min")
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    let grid = parse(&lines);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
