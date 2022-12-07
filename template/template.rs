use std::io;

fn part1(lines: &Vec<String>) -> usize {
    lines.len()
}

fn part2(lines: &Vec<String>) -> usize {
    lines.len()
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
