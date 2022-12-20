use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn parse(lines: &Vec<String>) -> Vec<isize> {
    lines
        .iter()
        .map(|s| s.parse::<isize>().expect("couldn't parse int"))
        .collect()
}

fn wrapped_mod(mut i: isize, len: usize) -> usize {
    while i < 0 {
        i += len as isize;
    }
    i as usize % len
}

fn part1(fields: &Vec<isize>) -> isize {
    let len = fields.len();

    let ret = fields
        .into_iter()
        .enumerate()
        .fold(
            (
                Vec::<(Range<usize>, isize)>::new(),
                Vec::<(usize, isize)>::new(),
            ),
            |(mut offsets, mut ret), (i, f)| {
                let wrapped_offset_i: usize =
                    offsets.iter().fold(i, |offset_i, (range, range_offset)| {
                        if range.contains(&offset_i) {
                            dbg!(wrapped_mod(offset_i as isize + range_offset, len))
                        } else {
                            offset_i
                        }
                    });
                let real_i: usize = wrapped_mod(
                    wrapped_offset_i as isize + if *f < 0 { *f - 1 } else { *f },
                    fields.len(),
                );
                let removal_offset: (Range<usize>, isize) = (wrapped_offset_i + 1..len, -1);

                /*
                if *f < -(wrapped_offset_i as isize) && removal_offset.0.contains(&real_i) {
                    real_i = (real_i as isize + removal_offset.1) as usize;
                }
                */

                println!(
                    "i: {} wrapped_offset_i: {} real_i: {}",
                    i, wrapped_offset_i, real_i
                );

                let insertion_offset: (Range<usize>, isize) = (real_i + 1..len, 1);
                let new_offsets = [removal_offset, insertion_offset];
                new_offsets.iter().for_each(|new_offset| {
                    ret.iter_mut().for_each(|mut field| {
                        if new_offset.0.contains(&field.0) {
                            field.0 = wrapped_mod(field.0 as isize + new_offset.1, len);
                        }
                    });
                });
                offsets.extend(new_offsets);
                ret.push((real_i, *f));
                ret.sort();
                (offsets, dbg!(ret))
            },
        )
        .1;
    let ret = ret.iter().fold(vec![0; len], |mut ret, (i, f)| {
        ret[*i] = *f;
        ret
    });
    let zero_pos = ret
        .iter()
        .position(|i| *i == 0)
        .expect("Couldn't find position of value 0");

    [1000, 2000, 3000]
        .into_iter()
        .map(|i| wrapped_mod(zero_pos as isize + i, len))
        .inspect(|i| println!("wrapped_i: {}", i))
        .map(|i| ret[i])
        .inspect(|i| println!("val: {}", i))
        .sum::<isize>()
}

fn part2(lines: &Vec<String>) -> usize {
    lines.len()
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

    let nums = parse(&lines);

    println!("Part 1: {}", part1(&nums));
    println!("Part 2: {}", part2(&lines));
}
