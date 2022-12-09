use colored::Colorize;
use once_cell::sync::Lazy;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::thread;
use std::time;
use std::{collections::HashSet, io};

static PRINT_GRID: bool = false;
static ANIMATE_GRID: bool = false;
static ANIMATION_SPEED: time::Duration = time::Duration::from_millis(20);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn parse_moves(lines: &Vec<String>) -> Vec<(Direction, usize)> {
    lines
        .iter()
        .map(|line| {
            let mut words = line.split_whitespace();
            let direction = words.next().expect("Couldn't get direction word");
            let count = words
                .next()
                .expect("Couldn't get count")
                .parse::<usize>()
                .expect("Couldn't parse count");
            match direction {
                "R" => Some((Direction::Right, count)),
                "L" => Some((Direction::Left, count)),
                "U" => Some((Direction::Up, count)),
                "D" => Some((Direction::Down, count)),
                _ => None,
            }
            .expect("Couldn't match direction")
        })
        .collect()
}

fn print_grid(
    head_pos: (i32, i32),
    tail_poss: &Vec<(i32, i32)>,
    visited: &HashSet<(i32, i32)>,
    visited_of_interest: &HashSet<(i32, i32)>,
) {
    static MAX_X: i32 = 31;
    static MIN_X: i32 = -MAX_X;
    static MAX_Y: i32 = 30;
    static MIN_Y: i32 = -MAX_Y;

    let mut covers = HashMap::<String, Vec<String>>::new();

    let reduce_pos = |getter: fn(&(i32, i32)) -> i32, reduce: &fn(i32, i32) -> i32| {
        reduce(
            getter(&head_pos),
            tail_poss
                .iter()
                .map(getter)
                .reduce(reduce)
                .expect("Couldn't get min tail pos"),
        )
    };

    let min_y_pos = reduce_pos(|pos| pos.1, &(min::<i32> as fn(i32, i32) -> i32));
    let max_y_pos = reduce_pos(|pos| pos.1, &(max::<i32> as fn(i32, i32) -> i32));
    let min_x_pos = reduce_pos(|pos| pos.0, &(min::<i32> as fn(i32, i32) -> i32));
    let max_x_pos = reduce_pos(|pos| pos.0, &(max::<i32> as fn(i32, i32) -> i32));
    for y in (min(min_y_pos, max(MIN_Y, MIN_Y - (MAX_Y - max_y_pos)))
        ..=max(max_y_pos, min(MAX_Y, MAX_Y - (MIN_Y - min_y_pos))))
        .rev()
    {
        for x in min(min_x_pos, max(MIN_X, MIN_X - (MAX_X - max_x_pos)))
            ..=max(max_x_pos, min(MAX_X, MAX_X - (MIN_X - min_x_pos)))
        {
            let matching_tail_poss = tail_poss
                .iter()
                .enumerate()
                .filter_map(|(i, pos)| {
                    if pos.0 == x && pos.1 == y {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let num_matching_tails = matching_tail_poss.len();
            if head_pos.0 == x && head_pos.1 == y {
                print!("{}", format!("H").bright_green());
                let mut covers_val: Vec<_> =
                    matching_tail_poss.iter().map(usize::to_string).collect();
                if x == 0 && y == 0 {
                    covers_val.push("s".to_string());
                }
                covers.insert("H".to_string(), covers_val);
            } else if num_matching_tails > 0 {
                let mut matching_tails = matching_tail_poss.iter();
                let tail_index = *matching_tails.next().expect("Couldn't get tail name");
                print!("{}", format!("{}", tail_index).bright_green());
                let mut covers_val: Vec<_> = matching_tails.map(usize::to_string).collect();
                if x == 0 && y == 0 {
                    covers_val.push("s".to_string());
                }
                covers.insert(tail_index.to_string(), covers_val);
            } else if x == 0 && y == 0 {
                static START_STR: Lazy<String> = Lazy::new(|| format!("s").red().to_string());
                print!("{}", *START_STR);
            } else {
                static INTEREST_STR: Lazy<String> =
                    Lazy::new(|| format!("X").bright_blue().to_string());
                static VISITED_STR: Lazy<String> = Lazy::new(|| format!("-").green().to_string());
                if visited_of_interest.contains(&(x, y)) {
                    print!("{}", *INTEREST_STR);
                } else if visited.contains(&(x, y)) {
                    print!("{}", *VISITED_STR);
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }
    println!(
        "{}",
        covers
            .iter()
            .filter_map(|(k, v)| if v.len() > 0 {
                Some(k.to_string() + " covers " + &v.join(", "))
            } else {
                None
            })
            .collect::<Vec<_>>()
            .join("; ")
    );
}

fn get_dir(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Right => (1, 0),
        Direction::Left => (-1, 0),
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
    }
}

fn add(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

fn sub(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 - rhs.0, lhs.1 - rhs.1)
}

fn dir(diff: (i32, i32)) -> (i32, i32) {
    (
        if diff.0 != 0 {
            diff.0 / i32::abs(diff.0)
        } else {
            0
        },
        if diff.1 != 0 {
            diff.1 / i32::abs(diff.1)
        } else {
            0
        },
    )
}

fn rope_sim(moves: &Vec<(Direction, usize)>, num_tails: usize, tail_of_interest: usize) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut visited_of_interest = HashSet::<(i32, i32)>::new();
    let mut head_pos = (0, 0);
    let mut tail_poss = vec![head_pos; num_tails];
    visited.insert(head_pos);
    visited_of_interest.insert(head_pos);

    if PRINT_GRID {
        print_grid(head_pos, &tail_poss, &visited, &visited_of_interest);
    }

    for (head_move, count) in moves {
        for _ in 0..*count {
            head_pos = add(head_pos, get_dir(*head_move));
            visited.insert(head_pos);

            let mut prev_pos = head_pos;
            for tail_pos in &mut tail_poss {
                let diff = sub(prev_pos, *tail_pos);

                if i32::abs(diff.0) >= 2 || i32::abs(diff.1) >= 2 {
                    *tail_pos = add(*tail_pos, dir(diff));
                    visited.insert(*tail_pos);
                }
                prev_pos = *tail_pos;
            }

            visited_of_interest.insert(tail_poss[tail_of_interest]);

            if ANIMATE_GRID {
                thread::sleep(ANIMATION_SPEED);
                println!("\x1Bc");
                print_grid(head_pos, &tail_poss, &visited, &visited_of_interest);
            }
        }
        if PRINT_GRID && !ANIMATE_GRID {
            print_grid(head_pos, &tail_poss, &visited, &visited_of_interest);
        }
    }

    if PRINT_GRID {
        print_grid(head_pos, &tail_poss, &visited, &visited_of_interest);
    }

    visited_of_interest.len()
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let moves = parse_moves(&lines);

    println!("Part 1: {}", rope_sim(&moves, 1, 0));
    println!("Part 2: {}", rope_sim(&moves, 9, 8));
}
