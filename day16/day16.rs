use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn parse(lines: &Vec<String>) -> (Vec<Valve>, HashMap<String, usize>) {
    static VALVE_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)")
            .unwrap()
    });

    let valves = lines
        .into_iter()
        .map(|line| -> Valve {
            let parsed_line = VALVE_RE
                .captures_iter(line)
                .next()
                .expect("Couldn't match regex");
            Valve {
                name: parsed_line[1].to_string(),
                flow_rate: parsed_line[2]
                    .parse::<usize>()
                    .expect("Couldn't parse flow rate"),
                tunnels: parsed_line[3]
                    .split(",")
                    .map(str::trim)
                    .map(str::to_string)
                    .collect(),
            }
        })
        .collect::<Vec<_>>();
    let names = valves
        .iter()
        .enumerate()
        .map(|(i, valve)| (valve.name.to_string(), i))
        .collect();
    (valves, names)
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct State {
    open_valves: HashSet<usize>,
    total_flow: usize,
    current_valve: usize,
}

impl State {
    fn new() -> State {
        State {
            open_valves: [].into_iter().collect(),
            total_flow: 0,
            current_valve: 0,
        }
    }
}

fn part1(valves: &Vec<Valve>, names: &HashMap<String, usize>, time: usize) -> usize {
    let start_room = names["AA"];
    let mut states = [State {
        open_valves: [].into_iter().collect(),
        total_flow: 0,
        current_valve: start_room,
    }]
    .into_iter()
    .collect::<Vec<_>>();
    for minute in 1..=time {
        println!("== Minute {} ==", minute);
        let mut new_states = Vec::<State>::new();
        for mut state in states.iter_mut() {
            //println!("State 1: {:?}", state);
            state.total_flow += state
                .open_valves
                .iter()
                .map(|open_valve| valves[*open_valve].flow_rate)
                .sum::<usize>();
            let current_valve: &Valve = &valves[state.current_valve];

            let mut new_state = state.clone();

            let reuse_state_offset = if !state.open_valves.contains(&state.current_valve)
                && current_valve.flow_rate > 0
            {
                state.open_valves.insert(state.current_valve.clone());
                0
            } else {
                let tunnel = current_valve
                    .tunnels
                    .iter()
                    .take(1)
                    .next()
                    .expect("Couldn't get first tunnel");
                state.current_valve = names[tunnel];
                1
            };

            new_states.extend(current_valve.tunnels.iter().skip(reuse_state_offset).map(
                |tunnel| {
                    let mut new_state = new_state.clone();
                    new_state.current_valve = names[tunnel];
                    new_state
                },
            ));
        }
        states.extend(new_states);
        states.sort_by(|s1, s2| usize::cmp(&s2.total_flow, &s1.total_flow));
        states.resize(min(2048, states.len()), State::new());
    }
    states
        .iter()
        .map(|state| state.total_flow)
        .max()
        .expect("Couldn't get max flow")
}

fn part2(lines: &Vec<String>) -> usize {
    lines.len()
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

    let (valves, names) = parse(&lines);

    println!("valves: {:?} names: {:?}", valves, names);

    println!("Part 1: {}", part1(&valves, &names, 30));
    println!("Part 2: {}", part2(&lines));
}
