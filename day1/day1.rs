use std::io::{self, BufRead};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut lines = io::stdin().lock().lines();

    let mut top_3_calorie_counts = BinaryHeap::new();

    let mut add_elf = |elf_count: &mut i32| {
        const TOP_N_ELVES: usize = 3;

        top_3_calorie_counts.push(Reverse(*elf_count));
        if top_3_calorie_counts.len() > TOP_N_ELVES {
            top_3_calorie_counts.pop();
        }

        *elf_count = 0;
    };

    let mut current_calorie_count = 0;

    while let Some(line) = lines.next() {
        let count = line.unwrap().parse::<i32>();
        if count.is_ok() {
            current_calorie_count += count.unwrap();
        }
        else {
            add_elf(&mut current_calorie_count);
        }
    }
    add_elf(&mut current_calorie_count);

    let sum: i32 = top_3_calorie_counts.iter()
        .map(|x| match x {
            Reverse(a) => *a
        }).sum();
    println!("total: {sum}");
}
