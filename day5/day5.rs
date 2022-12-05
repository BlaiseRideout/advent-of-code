use std::io;
use std::str;
//use std::collections::HashSet;

fn parse_crates(crates: &Vec<String>) -> Vec<Vec<char>> {
    let mut ret = Vec::<Vec<char>>::new();

    let cratevecs:Vec<_> = crates.iter().map(|crateline| {
        crateline.as_bytes().chunks(4)
            .map(str::from_utf8)
            .filter_map(Result::ok)
            .map(str::trim)
            .collect::<Vec<&str>>()
    })
    .rev().collect();
    for cratevec in cratevecs {
        while ret.len() < cratevec.len() {
            ret.push(Vec::<char>::new());
        }
        for (column_index, cratechunk) in cratevec.iter().enumerate() {
            if cratechunk.chars().nth(0) == Some('[') {
                ret[column_index].push(cratechunk.chars().nth(1).expect("Couldn't parse char"));
            }
        }
    }
    return ret;
}

fn parse_moves(moves: &Vec<String>) -> Vec<Vec<usize>> {
    moves.iter().map(|movestr| {
        movestr.split_whitespace()
                .map(|elem| elem.parse::<usize>())
                .filter_map(Result::ok)
                .collect()
    }).collect()
}

fn part1(mut crates: Vec<Vec<char>>, moves: &Vec<Vec<usize>>) -> String {
    for move_vec in moves {
        assert_eq!(move_vec.len(), 3);

        let mut popped_crates = (0..move_vec[0]).map(|_| {
            crates[move_vec[1] - 1].pop().expect("Couldn't pop crate")
        }).collect::<Vec<char>>();
        crates[move_vec[2] - 1].append(&mut popped_crates);
    }
    crates.iter()
        .map(|crate_stack| crate_stack.last().copied().expect("Couldn't get top of crate stack"))
        .collect::<String>()
}

fn part2(mut crates: Vec<Vec<char>>, moves: &Vec<Vec<usize>>) -> String {
    for move_vec in moves {
        assert_eq!(move_vec.len(), 3);
        let mut popped_crates = (0..move_vec[0]).map(|_| {
            crates[move_vec[1] - 1].pop().expect("Couldn't pop crate")
        })
        .collect::<Vec<char>>()
        .iter().rev().cloned().collect();
        crates[move_vec[2] - 1].append(&mut popped_crates);
    }
    crates.iter().map(|crate_stack| crate_stack.last().copied().expect("Couldn't get top of crate stack"))
        .collect::<String>()
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();

    let split_lines: Vec<Vec<String>> = lines.split(|line| line.is_empty())
                    .map(|region| region.iter()
                                 .map(|line| line.to_string())
                                 .collect()
                         ).collect();
    let crates_strs: &Vec<String> = &split_lines[0];
    let moves_strs: &Vec<String> = &split_lines[1];

    let crates = parse_crates(&crates_strs);
    let moves = parse_moves(&moves_strs);

    //println!("Starting crates: {:?}", crates);
    //println!("Moves: {:?}", moves);

    println!("Part 1: {:?}", part1(crates.iter().cloned().collect(), &moves));
    println!("Part 2: {:?}", part2(crates.iter().cloned().collect(), &moves));
}
