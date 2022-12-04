#[macro_use]
extern crate lazy_static;

use std::io;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Copy,Clone,PartialEq,Eq,Hash)]
enum Move {
    Rock = 1,
    Paper,
    Scissors,
}
impl FromStr for Move {
    type Err = ();
    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq)]
enum GameResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}
impl FromStr for GameResult {
    type Err = ();
    fn from_str(input: &str) -> Result<GameResult, Self::Err> {
        match input {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(()),
        }
    }
}

lazy_static! {
    static ref MOVE_THAT_BEATS: HashMap<Move, Move> = [
        (Move::Scissors, Move::Rock),
        (Move::Rock, Move::Paper),
        (Move::Paper, Move::Scissors),
    ].iter().cloned().collect();
}

fn part1(rounds: &Vec<String>) -> i32 {
     rounds.iter()
        .map(|round| -> i32 {
        let opponent_move = Move::from_str(&round[0..1]).expect("Couldn't parse opponent move");
        let player_move = Move::from_str(&round[2..3]).expect("Couldn't parse player move");
        player_move as i32 +
            if MOVE_THAT_BEATS[&opponent_move] == player_move { 6 }
            else if MOVE_THAT_BEATS[&player_move] == opponent_move { 0 }
            else { 3 }
    }).sum()
}

fn move_for_result_against(m: Move, r: GameResult) -> Move {
    match r {
        GameResult::Draw => m,
        GameResult::Win => MOVE_THAT_BEATS[&m],
        GameResult::Lose => *MOVE_THAT_BEATS.iter()
                            .find_map(|(key, val)|
                                      if *val == m { Some(key) }
                                      else { None }).unwrap(),
    }
}

fn part2(rounds: &Vec<String>) -> i32 {
     rounds.iter()
        .map(|round| -> i32 {
        let opponent_move = Move::from_str(&round[0..1]).expect("Couldn't parse opponent move");
        let target_result = GameResult::from_str(&round[2..3]).expect("Couldn't parse target result");
        let player_move = move_for_result_against(opponent_move, target_result);
        player_move as i32 + target_result as i32
    }).sum()
}

fn main() {
    let rounds: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();

    println!("Part 1: {:?}", part1(&rounds));
    println!("Part 2: {:?}", part2(&rounds));
}
