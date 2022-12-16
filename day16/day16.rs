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
    current_valves: Vec<usize>,
}

impl State {
    fn new(start_valve: usize, num_agents: usize) -> State {
        State {
            open_valves: [].into_iter().collect(),
            total_flow: 0,
            current_valves: vec![start_valve; num_agents],
        }
    }
}

fn simulate_valves(
    valves: &Vec<Valve>,
    names: &HashMap<String, usize>,
    time: usize,
    num_agents: usize,
    max_state_pool_size: usize,
) -> usize {
    let start_room = names["AA"];
    let mut states = vec![State::new(start_room, num_agents)];
    for minute in 1..=time {
        println!("== Minute {} ==", minute);

        for mut state in states.iter_mut() {
            state.total_flow += state
                .open_valves
                .iter()
                .map(|open_valve| valves[*open_valve].flow_rate)
                .sum::<usize>();
        }

        for i_agent in 0..num_agents {
            let mut new_states = Vec::<State>::new();
            for state in states.iter_mut() {
                let new_state = state.clone();

                let current_valve_index = &mut state.current_valves[i_agent];
                let current_valve = &valves[*current_valve_index];

                let reuse_state_offset = if !state.open_valves.contains(&current_valve_index)
                    && current_valve.flow_rate > 0
                {
                    state.open_valves.insert(current_valve_index.clone());
                    0
                } else {
                    let tunnel = current_valve
                        .tunnels
                        .iter()
                        .take(1)
                        .next()
                        .expect("Couldn't get first tunnel");
                    *current_valve_index = names[tunnel];
                    1
                };

                new_states.extend(current_valve.tunnels.iter().skip(reuse_state_offset).map(
                    |tunnel| {
                        let mut new_state = new_state.clone();
                        new_state.current_valves[i_agent] = names[tunnel];
                        new_state
                    },
                ));
            }
            states.extend(new_states);
        }
        states.sort_by(|s1, s2| usize::cmp(&s2.total_flow, &s1.total_flow));
        states.resize(
            min(max_state_pool_size, states.len()),
            State::new(start_room, num_agents),
        );
    }
    states
        .iter()
        .map(|state| state.total_flow)
        .max()
        .expect("Couldn't get max flow")
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

    println!("Part 1: {}", simulate_valves(&valves, &names, 30, 1, 2048));
    println!(
        "Part 2: {}",
        simulate_valves(&valves, &names, 26, 2, (2 as usize).pow(16))
    );
}
