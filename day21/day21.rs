use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
    Eq,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Monkey {
    Operation {
        op: Operation,
        lhs: String,
        rhs: String,
    },
    Number(isize),
}

fn parse(lines: &Vec<String>) -> HashMap<String, Monkey> {
    lines
        .iter()
        .map(|line| {
            let mut split_line = line.split(":").into_iter();
            let name = split_line
                .next()
                .expect("Couldn't get monkey name")
                .trim()
                .to_string();
            let monkey_type = split_line
                .next()
                .expect("Couldn't get monkey type/operation")
                .trim();
            let monkey_type_vals = monkey_type.split_whitespace().collect::<Vec<_>>();

            if monkey_type_vals.len() == 1 {
                let num = monkey_type_vals[0]
                    .parse::<isize>()
                    .expect("Couldn't parse monkey num as int");
                (name, Monkey::Number(num))
            } else if monkey_type_vals.len() == 3 {
                let lhs = monkey_type_vals[0].to_string();
                let rhs = monkey_type_vals[2].to_string();

                let op_type_str = monkey_type_vals[1];

                let op: Operation = match op_type_str {
                    "+" => Some(Operation::Add),
                    "*" => Some(Operation::Multiply),
                    "/" => Some(Operation::Divide),
                    "-" => Some(Operation::Subtract),
                    _ => None,
                }
                .expect("Couldn't parse operation type");
                (name, Monkey::Operation { op, lhs, rhs })
            } else {
                panic!("Monkey should have 1 or 3 fields");
            }
        })
        .collect()
}

fn do_op(op: Operation, lhs: isize, rhs: isize) -> isize {
    match op {
        Operation::Add => lhs + rhs,
        Operation::Subtract => lhs - rhs,
        Operation::Divide => lhs / rhs,
        Operation::Multiply => lhs * rhs,
        Operation::Eq => {
            if lhs < rhs {
                -1
            } else if lhs > rhs {
                1
            } else {
                0
            }
        }
    }
}

fn process_monkeys(monkeys: &HashMap<String, Monkey>) -> HashMap<String, isize> {
    let mut results: HashMap<String, isize> = monkeys
        .iter()
        .filter_map(|(name, monkey)| -> Option<(String, isize)> {
            match monkey {
                Monkey::Number(val) => Some((name.to_string(), *val)),
                _ => None,
            }
        })
        .collect();

    let mut to_process = vec!["root".to_string()];
    while !to_process.is_empty() {
        let curr = to_process
            .pop()
            .expect("Couldn't get top monkey to process");
        match &monkeys[curr.as_str()] {
            Monkey::Operation { op, rhs, lhs } => {
                let lhs_has_result = results.contains_key(lhs);
                let rhs_has_result = results.contains_key(rhs);
                if lhs_has_result && rhs_has_result {
                    let lhs = *results.get(lhs).expect("Couldn't get lhs result");
                    let rhs = *results.get(rhs).expect("Couldn't get rhs result");
                    results.insert(curr.to_string(), do_op(*op, lhs, rhs));
                } else {
                    to_process.push(curr);
                    if !results.contains_key(lhs) {
                        to_process.push(lhs.to_string());
                    }
                    if !results.contains_key(rhs) {
                        to_process.push(rhs.to_string());
                    }
                    continue;
                }
            }
            // We insert all the numbers up front
            Monkey::Number(_) => (),
        }
    }
    results
}

fn part1(monkeys: &HashMap<String, Monkey>) -> isize {
    let results = process_monkeys(&monkeys);

    assert!(results.contains_key("root"));

    results["root"]
}

fn part2(mut monkeys: HashMap<String, Monkey>) -> isize {
    if let Some(Monkey::Operation { op, lhs: _, rhs: _ }) = monkeys.get_mut("root") {
        *op = Operation::Eq;
    }

    static SEARCH_START: isize = 10isize.pow(14);
    let mut lower_bound: isize = -SEARCH_START;
    let mut upper_bound: isize = SEARCH_START;
    for invert_cmp in [true, false] {
        loop {
            let test_val = lower_bound + (upper_bound - lower_bound) / 2;
            if let Some(Monkey::Number(x)) = monkeys.get_mut("humn") {
                *x = test_val;
            }
            let results = process_monkeys(&monkeys);
            let result = results["root"];
            if result == 0 {
                return test_val as isize;
            } else if (!invert_cmp && result < 0) || (invert_cmp && result > 0) {
                lower_bound = test_val;
            } else {
                upper_bound = test_val;
            }
            if isize::abs(lower_bound - upper_bound) <= 1 {
                lower_bound = -SEARCH_START;
                upper_bound = SEARCH_START;
                break;
            }
        }
    }
    println!(
        "Part 2 doesn't seem to be between {} and {}",
        -SEARCH_START, SEARCH_START
    );
    0
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
    let monkeys = parse(&lines);

    //println!("Monkeys: {:?}", monkeys);

    println!("Part 1: {}", part1(&monkeys));
    println!("Part 2: {}", part2(monkeys));
}
