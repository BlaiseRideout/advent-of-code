use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Color {
    Blue,
    Red,
    Green,
}
#[derive(Debug, PartialEq, Eq)]
struct ParseColorErr;
impl FromStr for Color {
    type Err = ParseColorErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" | "Red" => Ok(Color::Red),
            "blue" | "Blue" => Ok(Color::Blue),
            "green" | "Green" => Ok(Color::Green),
            _ => Err(ParseColorErr),
        }
    }
}
type Round = HashMap<Color, usize>;

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

fn parse(lines: &Vec<String>) -> Vec<Game> {
    lines
        .into_iter()
        .map(|line| {
            let id = line
                .split(": ")
                .take(1)
                .exactly_one()
                .expect("Couldn't get game ID str")
                .split(" ")
                .skip(1)
                .take(1)
                .exactly_one()
                .expect("Couldn't get game ID")
                .parse::<usize>()
                .expect("Couldn't parse game ID as number");
            let rounds: Vec<Round> = line
                .split(": ")
                .skip(1)
                .take(1)
                .exactly_one()
                .expect("Couldn't get rounds for game")
                .split(";")
                .map(|round_str| {
                    round_str
                        .split(",")
                        .map(|cube_count_color| {
                            (
                                Color::from_str(
                                    cube_count_color
                                        .trim()
                                        .split(" ")
                                        .skip(1)
                                        .take(1)
                                        .exactly_one()
                                        .expect("Couldn't get color name"),
                                )
                                .expect("Couldn't parse color name"),
                                cube_count_color
                                    .trim()
                                    .split(" ")
                                    .take(1)
                                    .exactly_one()
                                    .expect("Couldn't get color count")
                                    .parse::<usize>()
                                    .expect("Couldn't parse color count"),
                            )
                        })
                        .collect::<Round>()
                })
                .collect();
            Game { id, rounds }
        })
        .collect()
}

fn part1(lines: &Vec<Game>) -> usize {
    let cube_counts: HashMap<Color, usize> =
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    lines
        .iter()
        .filter(|game| {
            !cube_counts.iter().any(|(&color, &count)| {
                game.rounds
                    .iter()
                    .any(|round| *round.get(&color).unwrap_or(&0) > count)
            })
        })
        .map(|game| game.id)
        .sum()
}

fn min_of_color(game: &Game, color: Color) -> usize {
    game.rounds
        .iter()
        .map(|round| *round.get(&color).unwrap_or(&0))
        .max()
        .expect("Couldn't get max for game")
}

fn part2(lines: &Vec<Game>) -> usize {
    lines
        .iter()
        .map(|game| {
            min_of_color(game, Color::Red)
                * min_of_color(game, Color::Green)
                * min_of_color(game, Color::Blue)
        })
        .sum()
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
    let parsed = parse(&lines);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}
