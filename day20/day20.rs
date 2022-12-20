use std::collections::{HashMap, HashSet};
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

fn simple_wrapped_mod(mut i: isize, len: usize) -> usize {
    while i < 0 {
        i += len as isize;
    }
    i as usize % len
}

fn find_wrapped_index(old_i: isize, mut offset: isize, len: usize) -> usize {
    if offset < 0 {
        offset += (len as isize - 1) * (-offset / (len as isize - 1) + 1);
    }
    if offset + old_i >= len as isize {
        offset -= (len as isize - 1) * ((offset + old_i) / (len as isize - 1));
    }
    let ret = (old_i + offset) as usize;
    assert!((0..len).contains(&ret));
    ret
}

fn mix_numbers(fields: &Vec<isize>, indices: &Vec<usize>) -> (Vec<isize>, Vec<usize>) {
    let len = fields.len();

    let ret = indices
        .into_iter()
        .fold(
            (
                Vec::<(Range<usize>, isize)>::new(),
                Vec::<(usize, isize)>::new(),
            ),
            |(mut offsets, mut ret), i| {
                let f = fields[*i];
                let wrapped_offset_i: usize =
                    offsets.iter().fold(*i, |offset_i, (range, range_offset)| {
                        if range.contains(&offset_i) {
                            simple_wrapped_mod(offset_i as isize + range_offset, len)
                        } else {
                            offset_i
                        }
                    });
                let real_i: usize = find_wrapped_index(wrapped_offset_i as isize, f, len);
                let removal_offset: (Range<usize>, isize) = (wrapped_offset_i + 1..len, -1);

                let insertion_offset: (Range<usize>, isize) = (real_i..len, 1);
                let new_offsets = [removal_offset, insertion_offset];
                new_offsets.iter().for_each(|new_offset| {
                    ret.iter_mut().for_each(|mut field| {
                        if new_offset.0.contains(&field.0) {
                            field.0 = simple_wrapped_mod(field.0 as isize + new_offset.1, len);
                        }
                    });
                });
                offsets.extend(new_offsets);
                ret.push((real_i, f));
                (offsets, ret)
            },
        )
        .1;
    (
        ret.iter().fold(vec![0; len], |mut ret, (i, f)| {
            ret[*i] = *f;
            ret
        }),
        ret.iter().map(|(i, _)| *i).collect(),
    )
}
fn mix_numbers_simple(fields: &Vec<isize>) -> (Vec<isize>, Vec<usize>) {
    mix_numbers(fields, &(0..fields.len()).collect())
}

fn grove_coordinates(mixed: &Vec<isize>) -> isize {
    let zero_pos = mixed
        .iter()
        .position(|i| *i == 0)
        .expect("Couldn't find position of value 0");
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| simple_wrapped_mod(zero_pos as isize + i, mixed.len()))
        //.inspect(|i| println!("wrapped_i: {}", i))
        .map(|i| mixed[i])
        //.inspect(|i| println!("val: {}", i))
        .sum::<isize>()
}

fn part1(fields: &Vec<isize>) -> isize {
    let (mixed, _) = mix_numbers_simple(fields);
    grove_coordinates(&mixed)
}

fn part2(fields: &Vec<isize>) -> isize {
    static DECRYPTION_KEY: isize = 811589153;
    static NUM_ITERATIONS: usize = 10;
    let mut mixed = fields
        .iter()
        .map(|x| x * DECRYPTION_KEY)
        .collect::<Vec<_>>();
    println!("Initial arrangement: \n{:?}", mixed);
    let mut indices: Vec<usize> = (0..fields.len()).collect();
    for iteration in 1..=NUM_ITERATIONS {
        (mixed, indices) = mix_numbers(&mixed, &indices);
        println!("After {} round of mixing: \n{:?}", iteration, mixed);
    }
    grove_coordinates(&mixed)
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
    //println!("Part 2: {}", part2(&nums));
}
