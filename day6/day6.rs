use std::io;
use std::collections::HashSet;

fn helper(line: &String, num_distinct: usize) -> usize {
    for i in 0..line.len() {
        if line[i..i+num_distinct].chars().collect::<HashSet<char>>().len() == num_distinct {
            return i+num_distinct;
        }
    }
    return 0;
}

fn main() {
    let line: String = io::stdin().lines().next()
                        .expect("Couldn't read input line").expect("Couldn't read input line")
                        .trim().to_string();

    println!("Part 1: {}", helper(&line, 4));
    println!("Part 2: {}", helper(&line, 14));
}
