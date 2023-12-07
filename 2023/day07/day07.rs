use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
enum Card {
  J,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  T,
  Q,
  K,
  A,
}

impl Card {
  fn from_char(c: char) -> Result<Card, ParseCardTypeError> {
    match c {
      '1' => Ok(Card::One),
      '2' => Ok(Card::Two),
      '3' => Ok(Card::Three),
      '4' => Ok(Card::Four),
      '5' => Ok(Card::Five),
      '6' => Ok(Card::Six),
      '7' => Ok(Card::Seven),
      '8' => Ok(Card::Eight),
      '9' => Ok(Card::Nine),
      'T' => Ok(Card::T),
      'J' => Ok(Card::J),
      'Q' => Ok(Card::Q),
      'K' => Ok(Card::K),
      'A' => Ok(Card::A),
      _ => Err(ParseCardTypeError),
    }
  }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct ParseCardTypeError;
impl FromStr for Card {
  type Err = ParseCardTypeError;
  fn from_str(s: &str) -> Result<Card, ParseCardTypeError> {
    Card::from_char(
      s.chars()
        .take(1)
        .exactly_one()
        .expect("Couldn't get char for card"),
    )
  }
}

type Hand = Vec<Card>;
fn parse(lines: &Vec<String>) -> Vec<(Hand, usize)> {
  lines
    .iter()
    .map(|line| {
      (
        line
          .split(" ")
          .take(1)
          .exactly_one()
          .expect("Couldn't get first half of line")
          .chars()
          .map(|c| Card::from_char(c).expect("Couldn't parse card in hand"))
          .collect(),
        line
          .split(" ")
          .skip(1)
          .take(1)
          .exactly_one()
          .expect("Couldn't get second half of line")
          .parse::<usize>()
          .expect("Couldn't parse hand value"),
      )
    })
    .collect()
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeKind,
  FullHouse,
  FourKind,
  FiveKind,
}

fn score_hand_type(hand: &Hand) -> Option<HandType> {
  let hand_set: HashSet<_> = hand.iter().cloned().collect();
  let hand_map: HashMap<Card, usize> = hand_set
    .iter()
    .map(|&card| (card, hand.into_iter().filter(|&c| *c == card).count()))
    .collect();
  if hand_set.len() == 1 {
    Some(HandType::FiveKind)
  } else if hand_set.len() == 2 {
    if hand_map.contains_key(&Card::J) {
      Some(HandType::FiveKind)
    } else if hand_map.iter().any(|(_, &count)| count == 4) {
      Some(HandType::FourKind)
    } else {
      Some(HandType::FullHouse)
    }
  } else if hand_set.len() == 5 {
    if hand_map.contains_key(&Card::J) {
      Some(HandType::OnePair)
    } else {
      Some(HandType::HighCard)
    }
  } else if hand_map.iter().any(|(_, &count)| count == 3) {
    if hand_map.contains_key(&Card::J) {
      Some(HandType::FourKind)
    } else {
      Some(HandType::ThreeKind)
    }
  } else if hand_map.iter().filter(|(_, &count)| count == 2).count() == 2 {
    if hand_map.contains_key(&Card::J) {
      if *hand_map.get(&Card::J).unwrap() == 2 {
        Some(HandType::FourKind)
      } else {
        Some(HandType::FullHouse)
      }
    } else {
      Some(HandType::TwoPair)
    }
  } else if hand_map.iter().filter(|(_, &count)| count == 2).count() == 1 {
    if hand_map.contains_key(&Card::J) {
      Some(HandType::ThreeKind)
    } else {
      Some(HandType::OnePair)
    }
  } else {
    None
  }
}

fn cmp_hands(h1: &Hand, h2: &Hand) -> Ordering {
  let t1 = score_hand_type(h1).expect("couldn't score h1");
  let t2 = score_hand_type(h2).expect("couldn't score h2");
  if t1 == t2 {
    let (v1, v2) = h1
      .into_iter()
      .interleave(h2.into_iter())
      .tuples()
      .find(|(&c1, &c2)| c1 != c2)
      .expect("Couldn't find tie break difference between hands");
    isize::cmp(&(*v1 as isize), &(*v2 as isize))
  } else {
    isize::cmp(&(t1 as isize), &(t2 as isize))
  }
}

fn part1(hands: &Vec<(Hand, usize)>) -> usize {
  hands
    .iter()
    .sorted_by(|h1, h2| cmp_hands(&h1.0, &h2.0))
    .enumerate()
    .map(|(rank, (_, bet))| (rank + 1) * bet)
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

  // p1
  // WRONG 251708371
  // WRONG 250345569
  // p2
  // WRONG 252782999
  println!("Part 2: {}", part1(&parsed));
}
