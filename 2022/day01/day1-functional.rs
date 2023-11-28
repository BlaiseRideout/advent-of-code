use std::io;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let lines: Vec<String> = io::stdin()
                                .lines()
                                .filter_map(Result::ok)
                                .collect();

    let elf_counts = lines
        .split(|line| line.is_empty())
        .map(|elf| elf.iter()
             .map(|snack| snack.parse::<i32>().unwrap())
             .sum::<i32>());

    let mut top_3_calorie_counts = BinaryHeap::new();
    for elf_count in elf_counts {
        top_3_calorie_counts.push(Reverse(elf_count));

        const TOP_N_ELVES: usize = 3;
        if top_3_calorie_counts.len() > TOP_N_ELVES {
            top_3_calorie_counts.pop();
        }
    }

    let sum: i32 = top_3_calorie_counts.iter()
        .map(|x| match x {
            Reverse(a) => *a
        }).sum();
    println!("total: {sum}");
}
