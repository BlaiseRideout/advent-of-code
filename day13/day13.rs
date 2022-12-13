use itertools::{EitherOrBoth::*, Itertools};
use serde_json::{json, Value};
use std::cmp;
use std::io;

fn parse_line(line: &String) -> Option<Value> {
    let line = line.trim();
    if line.len() != 0 {
        Some(serde_json::from_str(line).expect("Couldn't parse line"))
    } else {
        None
    }
}

fn parse_pt1(lines: &Vec<String>) -> Vec<Vec<Value>> {
    lines
        .chunks(3)
        .map(|chunk| chunk.iter().filter_map(parse_line).collect())
        .collect()
}

fn parse_pt2(lines: &Vec<String>) -> Vec<Value> {
    lines.iter().filter_map(parse_line).collect()
}

static DEBUG: bool = false;

// Returns true if lhs < rhs
fn ordered(lhs: &Value, rhs: &Value, depth: usize) -> cmp::Ordering {
    if DEBUG {
        print!("{: <1$}", "", depth * 2);
        println!(
            "- Compare: {} vs {}",
            serde_json::to_string(&lhs).expect("Couldn't serialize lhs"),
            serde_json::to_string(&rhs).expect("Couldn't serialize hhs")
        );
    }
    match lhs {
        Value::Number(x) => {
            let x: i64 = x.as_i64().expect("Couldn't parse int x");
            if let Value::Number(y) = rhs {
                let y = y.as_i64().expect("Couldn't parse int y");
                let res = i64::cmp(&x, &y);

                if DEBUG {
                    print!("{: <1$}", "", (depth + 1) * 2);
                    match res {
                        cmp::Ordering::Less => {
                            println!("- Left side is smaller, so inputs are in the RIGHT order")
                        }
                        cmp::Ordering::Greater => {
                            println!("- Right side is smaller, so inputs are in the WRONG order")
                        }
                        cmp::Ordering::Equal => {
                            println!("- Equal, so inputs are in the riGHT? order")
                        }
                    }
                }
                res
            } else {
                ordered(&json!(vec![x]), rhs, depth + 1)
            }
        }
        Value::Array(a) => {
            if let Value::Array(b) = rhs {
                a.iter().zip_longest(b.iter()).fold(
                    cmp::Ordering::Equal,
                    |ordering, pair| match ordering {
                        cmp::Ordering::Less => ordering,
                        cmp::Ordering::Greater => ordering,
                        _ => match pair {
                            Both(x, y) => {
                                let res = ordered(&x, &y, depth + 1);
                                if DEBUG {
                                    print!("{: <1$}", "", (depth + 1) * 2);
                                    match res {
                                        cmp::Ordering::Less => {
                                            println!("- Left side is smaller, so inputs are in the RIGHT order")
                                        }
                                        cmp::Ordering::Greater => {
                                            println!(
                                                "- Right side is smaller, so inputs are in the WRONG order"
                                            )
                                        }
                                        cmp::Ordering::Equal => {
                                            println!("- Both arrays are equal, so inputs are in the riGHT? order")
                                        }
                                    }
                                }
                                res
                            }

                            Right(_) => {
                                if DEBUG {
                                    print!("{: <1$}", "", (depth + 1) * 2);
                                    println!("- Left ran out of items first, so inputs are in the RIGHT order");
                                }
                                cmp::Ordering::Less
                            }
                            Left(_) => {
                                if DEBUG {
                                    print!("{: <1$}", "", (depth + 1) * 2);
                                    println!(
                                        "- Right ran out of items first, so inputs are in the WRONG order"
                                    );
                                }
                                cmp::Ordering::Greater
                            }
                        },
                    },
                )
            } else {
                if let Value::Number(y) = rhs {
                    ordered(lhs, &json!(vec![y]), depth + 1)
                } else {
                    panic!("Shouldn't be possible");
                }
            }
        }
        _ => cmp::Ordering::Equal,
    }
}

fn part1(parsed: &Vec<Vec<Value>>) -> usize {
    let ordered_pairs = parsed
        .iter()
        .enumerate()
        .map(|(i, pair)| {
            if DEBUG {
                println!("");
                println!("== Pair {} ==", i);
            }
            (i, ordered(&pair[0], &pair[1], 0))
        })
        .filter(|(_, b)| *b == cmp::Ordering::Less)
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();
    println!("ordered_pairs: {:?}", ordered_pairs);
    ordered_pairs.iter().sum()
}

fn part2(mut parsed: Vec<Value>) -> usize {
    parsed.sort_by(|lhs, rhs| ordered(lhs, rhs, 0));
    let poss: Vec<usize> = [vec![2], vec![6]]
        .iter()
        .map(|i| json!(i))
        .map(
            |divider| match parsed.binary_search_by(|i| ordered(i, &divider, 0)) {
                Ok(pos) => pos,
                Err(pos) => pos,
            },
        )
        .collect();
    (poss[0] + 1) * (poss[1] + 2)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let parsed_1 = parse_pt1(&lines);
    let parsed_2 = parse_pt2(&lines);
    if DEBUG {
        println!("parsed 1: {:?}", parsed_1);
    }

    println!("Part 1: {}", part1(&parsed_1));
    println!("Part 2: {}", part2(parsed_2));
}
