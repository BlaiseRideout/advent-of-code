use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn parse(lines: &Vec<String>) -> (String, HashMap<String, (String, String)>) {
  (
    lines
      .split(String::is_empty)
      .take(1)
      .exactly_one()
      .expect("Couldn't get directions")
      .into_iter()
      .take(1)
      .exactly_one()
      .expect("Couldn't get directions")
      .clone(),
    lines
      .split(String::is_empty)
      .skip(1)
      .take(1)
      .exactly_one()
      .expect("Couldn't get node list")
      .iter()
      .map(|node| -> (String, (String, String)) {
        (
          node
            .split(" = ")
            .take(1)
            .exactly_one()
            .expect("Couldn't get node name")
            .to_owned(),
          node
            .split(" = ")
            .skip(1)
            .take(1)
            .exactly_one()
            .expect("Couldn't get node edges")
            .strip_prefix("(")
            .expect("Couldn't strip open paren")
            .strip_suffix(")")
            .expect("Couldn't strip closing paren")
            .split(", ")
            .map(|s| String::from(s))
            .collect_tuple()
            .expect("Couldn't get tuple"),
        )
      })
      .collect(),
  )
}

fn part1(directions: &String, nodes: &HashMap<String, (String, String)>) -> usize {
  directions
    .chars()
    .cycle()
    .scan(
      nodes
        .keys()
        .find(|key| key.ends_with('A'))
        .expect("Couldn't find starting node for p1")
        .clone(),
      |cur_node, cur_direction| {
        let cur_edges = nodes
          .get(cur_node)
          .expect("Couldn't find current node in map");
        *cur_node = match cur_direction {
          'L' => cur_edges.0.clone(),
          _ => cur_edges.1.clone(),
        };
        if cur_node.ends_with('Z') {
          None
        } else {
          Some(0)
        }
      },
    )
    .count()
    + 1
}

// returns the start of the cycle, its length, and the indices of any ends nodes along the cycle
fn find_cycle_counts(
  start_node: &String,
  directions: &String,
  nodes: &HashMap<String, (String, String)>,
) -> usize {
  let steps = directions
    .chars()
    .cycle()
    .scan(
      (start_node.clone(), HashMap::<String, usize>::new(), 0usize),
      |(cur_node, visited_nodes, i), cur_direction| {
        if cur_node.ends_with('Z') && visited_nodes.contains_key(cur_node) {
          None
        } else {
          visited_nodes.insert(cur_node.clone(), *i);
          let cur_edges = nodes
            .get(cur_node)
            .expect("Couldn't find current node in map");
          *cur_node = match cur_direction {
            'L' => cur_edges.0.clone(),
            _ => cur_edges.1.clone(),
          };
          *i += 1;
          Some((cur_node.clone(), *i))
        }
      },
    )
    .collect_vec();
  let cycle_end_node = steps.last().expect("Couldn't get last node");
  let cycle_start_node = &steps
    .iter()
    .find(|(node, _)| *node == cycle_end_node.0)
    .expect("Couldn't find loop node");
  cycle_start_node.1
}

fn part2(directions: &String, nodes: &HashMap<String, (String, String)>) -> usize {
  let cycle_lengths = nodes
    .keys()
    .filter(|node| node.ends_with('A'))
    .map(|start_node| find_cycle_counts(start_node, directions, nodes))
    .collect_vec();
  let shortest_cycle = cycle_lengths.iter().min().expect("Couldn't get min cycle");
  (1..usize::MAX)
    .find(|x| {
      cycle_lengths
        .iter()
        .all(|cycle| (shortest_cycle * x) % cycle == 0)
    })
    .expect("Couldn't find cycle count")
    * shortest_cycle
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

  let (directions, nodes) = parse(&lines);

  println!("Part 1: {}", part1(&directions, &nodes));
  println!("Part 2: {}", part2(&directions, &nodes));
}
