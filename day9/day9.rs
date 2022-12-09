use std::cmp::{max, min};
use std::collections::HashMap;
use std::{collections::HashSet, io};

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

fn print_grid(head_pos: (i32, i32), tail_poss: &Vec<(i32, i32)>) {
    /*println!(
        "head_pos: {}, {} tail_pos: {}, {}",
        head_pos.0, head_pos.1, tail_pos.0, tail_pos.1
    );*/
    let min_bound = -10;
    let max_bound = 10;

    let mut covers = HashMap::<String, Vec<String>>::new();

    for y in (min(
        min(
            head_pos.1,
            tail_poss
                .iter()
                .map(|pos| pos.1)
                .min()
                .expect("Couldn't get min tail pos"),
        ),
        min_bound,
    )
        ..=max(
            max(
                head_pos.1,
                tail_poss
                    .iter()
                    .map(|pos| pos.1)
                    .max()
                    .expect("Couldn't get min tail pos"),
            ),
            max_bound,
        ))
        .rev()
    {
        for x in min(
            min(
                head_pos.0,
                tail_poss
                    .iter()
                    .map(|pos| pos.0)
                    .min()
                    .expect("Couldn't get min tail pos"),
            ),
            min_bound,
        )
            ..=max(
                max(
                    head_pos.0,
                    tail_poss
                        .iter()
                        .map(|pos| pos.0)
                        .max()
                        .expect("Couldn't get min tail pos"),
                ),
                max_bound,
            )
        {
            let matching_tail_pos = tail_poss
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
            let num_matching_tails = matching_tail_pos.len();
            if head_pos.0 == x && head_pos.1 == y {
                print!("H");
                let mut covers_val: Vec<_> =
                    matching_tail_pos.iter().map(usize::to_string).collect();
                if x == 0 && y == 0 {
                    covers_val.push("s".to_string());
                }
                covers.insert("H".to_string(), covers_val);
            } else if num_matching_tails > 0 {
                let mut matching_tails = matching_tail_pos.iter();
                let tail_index = *matching_tails.next().expect("Couldn't get tail name");
                print!("{}", tail_index);
                let mut covers_val: Vec<_> = matching_tails.map(usize::to_string).collect();
                if x == 0 && y == 0 {
                    covers_val.push("s".to_string());
                }
                covers.insert(tail_index.to_string(), covers_val);
            } else if x == 0 && y == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!("");
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
            .join("\n")
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

fn dist(diff: (i32, i32)) -> i32 {
    i32::abs(diff.0) + i32::abs(diff.1)
}

fn dir(diff: (i32, i32)) -> (i32, i32) {
    match diff {
        (0, 2) => (0, -1),
        (0, -2) => (0, 1),
        (2, 0) => (-1, 0),
        (-2, 0) => (1, 0),

        (2, 1) => (-1, -1),
        (-2, 1) => (1, -1),
        (2, -1) => (-1, 1),
        (-2, -1) => (1, 1),

        (1, 2) => (-1, -1),
        (1, -2) => (-1, 1),

        (-1, 2) => (1, -1),
        (-1, -2) => (1, 1),

        (-2, -2) => (1, 1),
        (-2, 2) => (1, -1),

        (2, 2) => (-1, -1),
        (2, -2) => (-1, 1),

        _ => (0, 0),
    }
    /*
    (
        if sub.0 != 0 { sub.0 / sub.0 } else { 0 },
        if sub.1 != 0 { sub.1 / sub.1 } else { 0 },
    )
    */
}

fn part1(moves: &Vec<(Direction, usize)>) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    visited.insert(tail_pos);
    for (head_move, count) in moves {
        for _ in 0..*count {
            head_pos = add(head_pos, get_dir(*head_move));

            let diff = sub(tail_pos, head_pos);
            /*println!(
                "diff: {:?}, dist(diff): {}, dir(diff): {:?}",
                diff,
                dist(diff),
                dir(diff)
            );*/
            tail_pos = add(tail_pos, dir(diff));
            visited.insert(tail_pos);
            //print_grid(head_pos, &vec![tail_pos]);
        }
    }
    visited.len()
}

fn part2(moves: &Vec<(Direction, usize)>, num_tails: usize, tail_of_interest: usize) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut head_pos = (0, 0);
    let mut tail_poss = vec![(0, 0); num_tails];
    visited.insert(tail_poss[tail_of_interest]);

    //print_grid(head_pos, &tail_poss);

    for (head_move, count) in moves {
        for _ in 0..*count {
            head_pos = add(head_pos, get_dir(*head_move));

            let mut prev_pos = head_pos;
            let mut test_print_grid = false;
            for tail_pos in &mut tail_poss {
                let diff = sub(*tail_pos, prev_pos);

                if i32::abs(diff.0) > 2 || i32::abs(diff.1) > 2 {
                    println!(
                        "diff: {:?}, dist(diff): {}, dir(diff): {:?}",
                        diff,
                        dist(diff),
                        dir(diff)
                    );

                    test_print_grid = true;
                }
                *tail_pos = add(*tail_pos, dir(diff));
                prev_pos = *tail_pos;
            }
            if test_print_grid {
                print_grid(head_pos, &tail_poss);
                break;
            }
            visited.insert(tail_poss[tail_of_interest]);
        }
        //print_grid(head_pos, &tail_poss);
    }
    //print_grid(head_pos, &tail_poss);
    visited.len()
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let moves = parse_moves(&lines);

    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves, 9, 8));
}
