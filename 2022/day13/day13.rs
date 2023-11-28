use itertools::{EitherOrBoth::*, Itertools};
use serde_json::{json, Value};
use std::cmp;
use std::io;

fn parse_lines(lines: &Vec<String>) -> Vec<Value> {
    lines
        .iter()
        .filter_map(|line| {
            let line = line.trim();
            if !line.is_empty() {
                Some(serde_json::from_str(line).expect("Couldn't parse line"))
            } else {
                None
            }
        })
        .collect()
}

fn order(lhs: &Value, rhs: &Value) -> cmp::Ordering {
    match (lhs, rhs) {
        (Value::Number(x), Value::Number(y)) => i64::cmp(
            &x.as_i64().expect("Couldn't parse int x"),
            &y.as_i64().expect("Couldn't parse int y"),
        ),
        (Value::Number(x), Value::Array(_)) => order(&json!(vec![x]), rhs),
        (Value::Array(_), Value::Number(y)) => order(lhs, &json!(vec![y])),
        (Value::Array(a), Value::Array(b)) => {
            a.iter()
                .zip_longest(b.iter())
                .fold(cmp::Ordering::Equal, |ordering, pair| match ordering {
                    cmp::Ordering::Less => ordering,
                    cmp::Ordering::Greater => ordering,
                    _ => match pair {
                        Both(x, y) => order(x, y),
                        Right(_) => cmp::Ordering::Less,
                        Left(_) => cmp::Ordering::Greater,
                    },
                })
        }
        _ => panic!("Values aren't arrays and numbers"),
    }
}

fn part1(parsed: &Vec<Value>) -> usize {
    parsed
        .into_iter()
        .tuples::<(_, _)>()
        .map(|pair| order(pair.0, pair.1))
        .enumerate()
        .filter(|(_, b)| *b == cmp::Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(mut parsed: Vec<Value>) -> usize {
    parsed.sort_by(order);
    [vec![2], vec![6]]
        .iter()
        .map(|i| json!(i))
        .enumerate()
        .map(|(i, divider)| {
            i + 1
                + match parsed.binary_search_by(|i| order(i, &divider)) {
                    Ok(pos) => pos,
                    Err(pos) => pos,
                }
        })
        .product()
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let parsed = parse_lines(&lines);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(parsed));
}
