use std::io;
use std::collections::HashSet;

fn helper(line: &String, num_distinct: usize) -> usize {
    line.as_bytes()
        .windows(num_distinct).enumerate()
        .find(|(_, window)| window.iter().collect::<HashSet<_>>().len() == num_distinct)
        .expect("Couldn't find valid window").0
    + num_distinct
}

fn main() {
    let line: String = io::stdin().lines().next()
                        .expect("Couldn't read input line").expect("Couldn't read input line")
                        .trim().to_string();

    println!("Part 1: {}", helper(&line, 4));
    println!("Part 2: {}", helper(&line, 14));
}
