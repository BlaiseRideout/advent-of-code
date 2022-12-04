use std::io;

fn parse_pairs(lines: &Vec<String>) -> Vec::<Vec::<Vec::<i32>>> {
    lines.iter().map(|line| -> Vec<Vec::<i32>> {
        line.split(",")
            .map(|range| range.split("-")
                 .map(|section| section.parse::<i32>()
                                  .expect("Couldn't parse int"))
                 .collect()
             ).collect()
    }).collect()
}

fn part1(lines: &Vec<String>) -> usize {
    parse_pairs(lines).iter()
        .filter(|line| -> bool {
            (&line[0][0] >= &line[1][0] && &line[0][1] <= &line[1][1]) ||
            (&line[1][0] >= &line[0][0] && &line[1][1] <= &line[0][1])
        }).count()
}

fn part2(lines: &Vec<String>) -> usize {
    parse_pairs(lines).iter()
        .filter(|line| -> bool {
            (&line[0][0] >= &line[1][0] && &line[0][0] <= &line[1][1]) ||
            (&line[0][1] >= &line[1][0] && &line[0][1] <= &line[1][1]) ||
            (&line[1][0] >= &line[0][0] && &line[1][0] <= &line[0][1]) ||
            (&line[1][1] >= &line[0][0] && &line[1][1] <= &line[0][1])
        }).count()
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    println!("Part 1: {:?}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
