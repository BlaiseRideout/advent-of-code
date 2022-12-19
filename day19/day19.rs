use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::{min, Reverse};
use std::convert::identity;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct ObsidianCost {
    ore: usize,
    clay: usize,
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct GeodeCost {
    ore: usize,
    obsidian: usize,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Blueprint {
    id: usize,
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: ObsidianCost,
    geode_cost: GeodeCost,
}

fn parse(lines: &Vec<String>) -> Vec<Blueprint> {
    fn parse_line(line: &String) -> Blueprint {
        static LINE_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new([r"Blueprint (?P<id>\d+):",
                        r"Each ore robot costs (?P<ore>\d+) ore.",
                        r"Each clay robot costs (?P<clay>\d+) ore.",
                        r"Each obsidian robot costs (?P<obsidian_ore>\d+) ore and (?P<obsidian_clay>\d+) clay.",
                        r"Each geode robot costs (?P<geode_ore>\d+) ore and (?P<geode_obsidian>\d+) obsidian."]
                        .iter()
                        .cloned()
                        .intersperse(r"\s+")
                        .collect::<String>().as_str()
                       ).unwrap()
        });
        let capts = LINE_RE
            .captures(&line)
            .expect("Couldn't parse line with regex");
        Blueprint {
            id: capts.name("id").unwrap().as_str().parse().unwrap(),
            ore_cost: capts.name("ore").unwrap().as_str().parse().unwrap(),
            clay_cost: capts.name("clay").unwrap().as_str().parse().unwrap(),
            obsidian_cost: ObsidianCost {
                ore: capts
                    .name("obsidian_ore")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
                clay: capts
                    .name("obsidian_clay")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            },
            geode_cost: GeodeCost {
                ore: capts.name("geode_ore").unwrap().as_str().parse().unwrap(),
                obsidian: capts
                    .name("geode_obsidian")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            },
        }
    }
    if lines.iter().any(String::is_empty) {
        lines
            .split(String::is_empty)
            .map(|lines| parse_line(&lines.join(" ")))
            .collect()
    } else {
        lines.iter().map(parse_line).collect()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,

    ore_bots_in_progress: usize,
    clay_bots_in_progress: usize,
    obsidian_bots_in_progress: usize,
    geode_bots_in_progress: usize,

    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl State {
    fn new() -> State {
        State {
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,

            ore_bots_in_progress: 0,
            clay_bots_in_progress: 0,
            obsidian_bots_in_progress: 0,
            geode_bots_in_progress: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
}

fn run_blueprint(blueprint: &Blueprint, time: usize, max_state_pool_size: usize) -> usize {
    let mut states = vec![State::new()];
    for minute in 1..=time {
        //println!("== Minute {} (Blueprint {}) == ", minute, blueprint.id);

        let mut new_states: Vec<State> = vec![];
        for state in states.iter() {
            new_states.extend(
                [
                    if state.ore >= blueprint.ore_cost {
                        let mut new_state: State = *state;
                        new_state.ore -= blueprint.ore_cost;
                        new_state.ore_bots_in_progress += 1;
                        Some(new_state)
                    } else {
                        None
                    },
                    if state.ore >= blueprint.clay_cost {
                        let mut new_state: State = *state;
                        new_state.ore -= blueprint.clay_cost;
                        new_state.clay_bots_in_progress += 1;
                        Some(new_state)
                    } else {
                        None
                    },
                    if state.ore >= blueprint.obsidian_cost.ore
                        && state.clay >= blueprint.obsidian_cost.clay
                    {
                        let mut new_state: State = *state;
                        new_state.ore -= blueprint.obsidian_cost.ore;
                        new_state.clay -= blueprint.obsidian_cost.clay;
                        new_state.obsidian_bots_in_progress += 1;
                        Some(new_state)
                    } else {
                        None
                    },
                    if state.ore >= blueprint.geode_cost.ore
                        && state.obsidian >= blueprint.geode_cost.obsidian
                    {
                        let mut new_state: State = *state;
                        new_state.ore -= blueprint.geode_cost.ore;
                        new_state.obsidian -= blueprint.geode_cost.obsidian;
                        new_state.geode_bots_in_progress += 1;
                        Some(new_state)
                    } else {
                        None
                    },
                ]
                .into_iter()
                .filter_map(identity),
            );
        }
        states.extend(new_states);

        for state in states.iter_mut() {
            state.ore += state.ore_bots;
            state.clay += state.clay_bots;
            state.obsidian += state.obsidian_bots;
            state.geodes += state.geode_bots;

            state.ore_bots += state.ore_bots_in_progress;
            state.clay_bots += state.clay_bots_in_progress;
            state.obsidian_bots += state.obsidian_bots_in_progress;
            state.geode_bots += state.geode_bots_in_progress;

            state.ore_bots_in_progress = 0;
            state.clay_bots_in_progress = 0;
            state.obsidian_bots_in_progress = 0;
            state.geode_bots_in_progress = 0;
        }

        states.sort_by_key(|state| {
            Reverse((
                state.geodes,
                state.geode_bots,
                state.geode_bots_in_progress,
                state.obsidian,
                state.obsidian_bots,
                state.obsidian_bots_in_progress,
                state.clay,
                state.clay_bots,
                state.clay_bots_in_progress,
                state.ore,
                state.ore_bots,
                state.ore_bots_in_progress,
            ))
        });
        states.resize(min(max_state_pool_size, states.len()), State::new());

        //println!("State 1: {:?}", states[0]);
    }
    states
        .into_iter()
        .map(|state| state.geodes)
        .max()
        .expect("Couldn't get largest geode count")
}

fn part1(blueprints: &Vec<Blueprint>) -> usize {
    static MINUTES: usize = 24;
    static POOL_SIZE: usize = (2 as usize).pow(8);
    blueprints
        .iter()
        .map(|blueprint| run_blueprint(&blueprint, MINUTES, POOL_SIZE) * blueprint.id)
        .sum()
}

fn part2(blueprints: &Vec<Blueprint>) -> usize {
    static MINUTES: usize = 32;
    static POOL_SIZE: usize = (2 as usize).pow(16);
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| run_blueprint(&blueprint, MINUTES, POOL_SIZE))
        .product()
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

    let blueprints = parse(&lines);

    println!("Part 1: {}", part1(&blueprints));
    println!("Part 2: {}", part2(&blueprints));
}
