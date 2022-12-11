use std::io;
use std::str;

fn transpose<T: Copy>(v: Vec<Vec<T>>, default_val: T) -> Vec<Vec<T>> {
    assert!(!v.is_empty());

    let len = v.iter().map(Vec::<_>::len).max().expect("Couldn't get transpose width");

    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap_or(default_val))
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_crates(crates: &Vec<String>) -> Vec<Vec<char>> {
    let mut cratevecs:Vec<_> = crates.iter()
        // First line is the top of the stack
        .rev()
        // Remove the bottom indices line
        .skip(1)
        .map(|crateline| {
            crateline.as_bytes().chunks(4)
                .map(str::from_utf8)
                .filter_map(Result::ok)
                .map(|cratestr|
                     match cratestr
                           .trim_matches(&[' ', '[', ']'] as &[_])
                           .chars().nth(0) {
                         Some(c) => c,
                         _ => '\0',
                     })
                .collect::<Vec<char>>()
        }).collect();
    // Transpose so our stack columns are their own Vecs we can push to/pop from
    cratevecs = transpose(cratevecs, '\0');
    cratevecs.iter_mut().for_each(|stack| stack.retain(|c| *c != '\0'));
    return cratevecs;
}

fn parse_moves(moves: &Vec<String>) -> Vec<Vec<usize>> {
    moves.iter().map(|movestr| {
        movestr.split_whitespace()
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .collect()
    }).collect()
}

fn part1(mut crates: Vec<Vec<char>>, moves: &Vec<Vec<usize>>) -> String {
    for move_vec in moves {
        assert_eq!(move_vec.len(), 3);

        let mut popped_crates =
            (0..move_vec[0]).map(|_| {
                crates[move_vec[1] - 1].pop().expect("Couldn't pop crate")
            }).collect::<Vec<char>>();
        crates[move_vec[2] - 1].append(&mut popped_crates);
    }
    crates.iter()
        .map(|crate_stack| crate_stack.last()
                             .copied().expect("Couldn't get top of crate stack"))
        .collect::<String>()
}

fn part2(mut crates: Vec<Vec<char>>, moves: &Vec<Vec<usize>>) -> String {
    for move_vec in moves {
        assert_eq!(move_vec.len(), 3);
        let mut popped_crates: Vec<char> =
            (0..move_vec[0]).map(|_| {
                crates[move_vec[1] - 1].pop().expect("Couldn't pop crate")
            })
            .collect::<Vec<char>>()
            .into_iter().rev().collect();
        crates[move_vec[2] - 1].append(&mut popped_crates);
    }
    crates.iter()
        .map(|crate_stack| crate_stack.last()
                             .copied().expect("Couldn't get top of crate stack"))
        .collect::<String>()
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();

    let mut line_regions: Vec<Vec<String>> =
        lines.split(String::is_empty)
             .map(|region| region.iter()
                             .map(String::clone)
                             .collect()
             ).collect();
    assert_eq!(line_regions.len(), 2);
    let moves_strs: Vec<String> = line_regions.pop().expect("Couldn't get moves");
    let crates_strs: Vec<String> = line_regions.pop().expect("Couldn't get crates");

    let crates = parse_crates(&crates_strs);
    let moves = parse_moves(&moves_strs);

    //println!("Starting crates: {:?}", crates);
    //println!("Moves: {:?}", moves);

    println!("Part 1: {}", part1(crates.to_vec(), &moves));
    println!("Part 2: {}", part2(crates.to_vec(), &moves));
}
