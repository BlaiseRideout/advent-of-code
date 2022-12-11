use std::io;
use std::collections::HashSet;

fn parse_pairs(lines: &Vec<String>) -> Vec<Vec<HashSet<i32>>> {
    lines.iter().map(|line| -> Vec<_> {
        line.split(",")
            .map(|range| range.split("-")
                 .map(|section| section.parse::<i32>()
                                  .expect("Couldn't parse int"))
                 .collect()
             )
             .map(|section: Vec<_>|
                      (section[0]..=section[1]).collect::<HashSet<_>>())
             .collect()
    }).collect()
}

fn part1(lines: &Vec<String>) -> usize {
    parse_pairs(lines).iter()
        .filter(|line|
            line[0].is_superset(&line[1]) || line[1].is_superset(&line[0])
        ).count()
}

fn part2(lines: &Vec<String>) -> usize {
    parse_pairs(lines).iter()
        .filter(|line| !line[0].is_disjoint(&line[1])).count()
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    println!("Part 1: {:?}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
