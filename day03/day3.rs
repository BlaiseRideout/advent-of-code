use std::io;
use std::collections::HashSet;

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => 0
    }
}

fn part1(sacks: &Vec<String>) -> u32 {
    sacks.iter().map(|sack| {
        let pocket_size = sack.len() / 2;
        let first_pocket_items: HashSet<char> = (&sack[0..pocket_size]).chars().collect();
        let second_pocket_items: HashSet<char> = (&sack[pocket_size..sack.len()]).chars().collect();
        *first_pocket_items.intersection(&second_pocket_items)
            .next().expect("Couldn't find duplicate item")
    }).map(priority).sum()
}

fn part2(sacks: &Vec<String>) -> u32 {
    sacks.chunks(3)
        .map(|group| {
            group.iter()
                .map(|sack| -> HashSet::<char> { sack.chars().collect() })
                .reduce(|s1, s2| { s1.intersection(&s2).cloned().collect() })
                .map(|item_set| *item_set.iter().next().expect("Couldn't find group item"))
                .expect("Couldn't find group item")
        }).map(priority).sum()
}

fn main() {
    let sacks: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    println!("Part 1: {}", part1(&sacks));
    println!("Part 2: {}", part2(&sacks));
}
