use once_cell::sync::Lazy;
use regex::Regex;
use std::io;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Operation {
    Add { value: Option<u64> },
    Subtract { value: Option<u64> },
    Divide { value: Option<u64> },
    Multiply { value: Option<u64> },
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: u64,
    test_result: (usize, usize),
}

fn parse_monkeys(lines: &Vec<String>) -> Vec<Monkey> {
    lines
        .chunks(7)
        .map(|monkey_lines| -> Monkey {
            let mut it = monkey_lines.iter();
            // Skip first monkey line
            it.next();
            let starting_str = it.next().expect("Couldn't get starting items");
            let starting_strs = starting_str
                .split(":")
                .skip(1)
                .next()
                .expect("Couldn't get starting item list")
                .trim()
                .split(", ");
            let items: Vec<u64> = starting_strs
                .map(str::parse::<u64>)
                .filter_map(Result::ok)
                .collect();
            static OP_RE: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"Operation: new = old (\+|-|\*|/) (\d+|old)").unwrap());
            let op_str = it.next().expect("Couldn't get operation").trim();
            let parsed_op = OP_RE
                .captures_iter(op_str)
                .next()
                .expect("Couldn't parse operation regex");
            let op_val = parsed_op[2].parse::<u64>().ok();
            let op: Operation = match &parsed_op[1] {
                "+" => Some(Operation::Add { value: op_val }),
                "*" => Some(Operation::Multiply { value: op_val }),
                "/" => Some(Operation::Divide { value: op_val }),
                "-" => Some(Operation::Subtract { value: op_val }),
                _ => None,
            }
            .expect("Couldn't parse operation type");

            static TEST_RE: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"Test: divisible by (\d+)").unwrap());
            let test_str = it.next().expect("Couldn't get test").trim();
            let parsed_test = TEST_RE
                .captures_iter(test_str)
                .next()
                .expect("Couldn't parse test regex");

            let test: u64 = parsed_test[1]
                .parse::<u64>()
                .expect("Couldn't parse test num");

            static TEST_RESULT_RE: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"If (true|false): throw to monkey (\d+)").unwrap());

            let result_strs = (
                it.next().expect("Couldn't get result 1").trim(),
                it.next().expect("Couldn't get result 2").trim(),
            );
            let parsed_results = (
                TEST_RESULT_RE
                    .captures_iter(result_strs.0)
                    .next()
                    .expect("Couldn't parse result regex 1"),
                TEST_RESULT_RE
                    .captures_iter(result_strs.1)
                    .next()
                    .expect("Couldn't parse result regex 2"),
            );

            let test_result: (usize, usize) = (
                parsed_results.0[2]
                    .parse::<usize>()
                    .expect("Couldn't parse result num"),
                parsed_results.1[2]
                    .parse::<usize>()
                    .expect("Couldn't parse result num"),
            );
            Monkey {
                items,
                op,
                test,
                test_result,
            }
        })
        .collect()
}

fn do_round(monkeys: &mut Vec<Monkey>, worry_reduction: u64, monkey_activity: &mut Vec<usize>) {
    let mod_val = monkeys.iter().fold(1 as u64, |v, monkey| v * monkey.test);
    for i in 0..monkeys.len() {
        let monkey: &mut Monkey = monkeys.get_mut(i).expect("Couldn't get monkey");
        let destination_monkeys = monkey
            .items
            .drain(0..monkey.items.len())
            .map(|mut item| {
                match monkey.op {
                    Operation::Add { value } => item += value.unwrap_or(item),
                    Operation::Subtract { value } => item -= value.unwrap_or(item),
                    Operation::Divide { value } => item /= value.unwrap_or(item),
                    Operation::Multiply { value } => item *= value.unwrap_or(item),
                }
                item /= worry_reduction;
                if item % monkey.test == 0 {
                    (monkey.test_result.0, item % mod_val)
                } else {
                    (monkey.test_result.1, item % mod_val)
                }
            })
            .collect::<Vec<_>>();

        *monkey_activity
            .get_mut(i)
            .expect("Couldn't update monkey activity") += destination_monkeys.len();

        //println!("{:?}", destination_monkeys);
        destination_monkeys
            .into_iter()
            .for_each(|(destination_monkey, item)| {
                monkeys
                    .get_mut(destination_monkey)
                    .expect("Couldn't get destination monkey")
                    .items
                    .push(item);
            });
    }
}

fn part1(mut monkeys: Vec<Monkey>) -> usize {
    let mut monkey_activity: Vec<usize> = vec![0; monkeys.len()];

    //println!("Monkeys: {:?}", monkeys);
    static NUM_ROUNDS: usize = 20;
    for _ in 0..NUM_ROUNDS {
        do_round(&mut monkeys, 3, &mut monkey_activity);
        //println!("Monkeys after round {}: {:?}", i, monkeys);
    }
    let mut top_monkeys = monkey_activity.iter().collect::<Vec<_>>();
    top_monkeys.sort_by(|a, b| usize::cmp(b, a));
    monkey_activity
        .iter()
        .enumerate()
        .for_each(|(i, activity)| {
            println!("Monkey {} inspected {} items", i, activity);
        });
    top_monkeys[0] * top_monkeys[1]
}

fn part2(mut monkeys: Vec<Monkey>) -> usize {
    let mut monkey_activity: Vec<usize> = vec![0; monkeys.len()];

    //println!("Monkeys: {:?}", monkeys);
    static NUM_ROUNDS: usize = 10_000;
    for _ in 0..NUM_ROUNDS {
        do_round(&mut monkeys, 1, &mut monkey_activity);
        //println!("Monkeys after round {}: {:?}", i, monkeys);
    }
    let mut top_monkeys = monkey_activity.iter().collect::<Vec<_>>();
    top_monkeys.sort_by(|a, b| usize::cmp(b, a));
    monkey_activity
        .iter()
        .enumerate()
        .for_each(|(i, activity)| {
            println!("Monkey {} inspected {} items", i, activity);
        });
    top_monkeys[0] * top_monkeys[1]
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let monkeys = parse_monkeys(&lines);

    println!("Part 1: {}", part1(monkeys.to_vec()));
    println!("Part 2: {}", part2(monkeys.to_vec()));
}
