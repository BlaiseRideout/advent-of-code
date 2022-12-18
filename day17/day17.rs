use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::thread;
use std::time::{self, Instant};

type RockShape = Vec<Vec<bool>>;

fn parse_rock_shapes(lines: &Vec<String>) -> Vec<RockShape> {
    lines
        .split(String::is_empty)
        .map(|rock_lines| {
            rock_lines
                .iter()
                .map(|line| line.chars().map(|c| c == '#').collect())
                //.rev() // reverse to match the order of grid
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
}

fn parse_jet_input(line: &String) -> Vec<Direction> {
    line.chars()
        .filter_map(|c| match c {
            '>' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            _ => None,
        })
        .collect()
}

// Grid is reversed, the last items are the highest
type Grid = Vec<Vec<bool>>;

fn print_grid(grid: &Grid) {
    grid.iter().rev().for_each(|row| {
        println!(
            "|{}|",
            row.into_iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        );
    });
}

static GRID_WIDTH: usize = 7;

static ANIMATION_SPEED: time::Duration = time::Duration::from_millis(200);
static ANIMATE: bool = false;
static VERBOSE: bool = false;
static DEBUG: bool = false;

fn print_grid_with_rock(grid: &Grid, current_rock: &RockShape, bottom_y: usize, left_x: usize) {
    let rock_height = current_rock.len();
    let rock_width = current_rock[0].len();

    let maxy = max(grid.len(), bottom_y + current_rock.len());
    if ANIMATE {
        thread::sleep(ANIMATION_SPEED);
        println!("\x1Bc");
    }
    for y in (0..maxy).rev() {
        print!("|");
        for x in 0..GRID_WIDTH {
            let rock_range_y = bottom_y..bottom_y + rock_height;
            let rock_range_x = left_x..left_x + rock_width;
            if (0..grid.len()).contains(&y) && grid[y][x] {
                print!("#");
            } else if rock_range_y.contains(&y)
                && rock_range_x.contains(&x)
                && current_rock[y - bottom_y][x - left_x]
            {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
    println!("");
}

fn simulate_rocks(
    rock_sequence: &Vec<RockShape>,
    jet_sequence: &Vec<Direction>,
    num_rocks: usize,
) -> usize {
    let mut grid: Grid = vec![];
    let mut tick_count = 0;
    let mut current_jet_in_sequence = 0;

    let mut cleared_row_offset: usize = 0;

    let start = Instant::now();

    let mut repeat_lengths = HashSet::<usize>::new();

    //static REPEAT_START_ROCK: usize = 7200; // example
    static REPEAT_START_ROCK: usize = 398200; // input

    //static REPEAT_LENGTH: usize = 200; // for searching
    //static REPEAT_LENGTH: usize = 7000; // example
    static REPEAT_LENGTH: usize = 745200 - REPEAT_START_ROCK; // input

    let mut repeat_start_height = 0;
    let mut repeat_end_height = 0;

    for i_rock in 0..=num_rocks {
        let current_rock_in_sequence = i_rock % rock_sequence.len();

        let highest_y = cleared_row_offset + grid.len() as usize;

        let current_rock = &rock_sequence[current_rock_in_sequence];

        let mut bottom_y = highest_y + 3;
        let mut left_x = 2;

        if VERBOSE {
            println!("A new rock begins falling:");
            print_grid_with_rock(
                &grid,
                &current_rock,
                (bottom_y - cleared_row_offset) as usize,
                left_x,
            );
        }

        loop {
            let jet_dir = jet_sequence[current_jet_in_sequence];
            let rock_blocked_at_coord = |left_x: usize, bottom_y: usize| -> bool {
                current_rock
                    .iter()
                    .rev() // grid y is reversed
                    .enumerate()
                    .any(|(y, row)| {
                        row.iter().enumerate().any(|(x, b)| {
                            *b && grid.len() as usize + cleared_row_offset > y as usize + bottom_y
                                && grid[0].len() > x + left_x
                                && grid[(y as usize + bottom_y - cleared_row_offset) as usize]
                                    [x + left_x]
                        })
                    })
            };
            match jet_dir {
                Direction::Right => {
                    let rock_width = current_rock[0].len();
                    let right_x = left_x + rock_width - 1;
                    // TODO: Add check for existing rocks
                    if right_x + 1 < GRID_WIDTH && !rock_blocked_at_coord(left_x + 1, bottom_y) {
                        if VERBOSE {
                            println!("Jet of gas pushes rock right:");
                        }
                        left_x += 1;
                    } else {
                        if VERBOSE {
                            println!("Jet of gas pushes rock right, but nothing happens:");
                        }
                    }
                }
                Direction::Left => {
                    // TODO: Add check for existing rocks
                    if left_x > 0 && !rock_blocked_at_coord(left_x - 1, bottom_y) {
                        if VERBOSE {
                            println!("Jet of gas pushes rock left:");
                        }
                        left_x -= 1;
                    } else {
                        if VERBOSE {
                            println!("Jet of gas pushes rock left, but nothing happens:");
                        }
                    }
                }
            }

            if VERBOSE {
                print_grid_with_rock(
                    &grid,
                    &current_rock,
                    (bottom_y - cleared_row_offset) as usize,
                    left_x,
                );
            }

            tick_count += 1;
            current_jet_in_sequence = tick_count % jet_sequence.len();

            if bottom_y == 0 || rock_blocked_at_coord(left_x, bottom_y - 1) {
                let rock_height = current_rock.len();
                grid.resize(
                    max(
                        rock_height + (bottom_y - cleared_row_offset) as usize,
                        grid.len(),
                    ),
                    vec![false; GRID_WIDTH],
                );
                current_rock
                    .iter()
                    .rev() // grid is reversed in y
                    .enumerate()
                    .for_each(|(y, row)| {
                        row.iter().enumerate().for_each(|(x, b)| {
                            if *b {
                                let grid_pos = &mut grid
                                    [y + (bottom_y - cleared_row_offset) as usize][x + left_x];
                                *grid_pos = *b;
                            }
                        })
                    });
                if VERBOSE {
                    println!("Rock falls 1 unit, causing it to come to rest:");
                    print_grid_with_rock(
                        &grid,
                        &current_rock,
                        (bottom_y - cleared_row_offset) as usize,
                        left_x,
                    );
                }
                break;
            } else {
                if VERBOSE {
                    println!("Rock falls 1 unit:");
                }
                bottom_y -= 1;
            }

            if VERBOSE {
                print_grid_with_rock(
                    &grid,
                    &current_rock,
                    (bottom_y - cleared_row_offset) as usize,
                    left_x,
                );
            }
        }

        if grid.len() > 1024 {
            let full_row = grid.len()
                - 10
                - (0..GRID_WIDTH)
                    .into_iter()
                    .map(|x| {
                        grid.iter()
                            .rev()
                            .position(|row| row[x])
                            .expect("Couldn't find filled position in column")
                    })
                    .max()
                    .expect("Couldn't get highest filled position");

            grid = grid[full_row..].to_vec();
            cleared_row_offset += full_row as usize;
        }

        if DEBUG {
            println!("Grid after {} rocks:", i_rock + 1);
            print_grid(&grid);
        }
        if num_rocks > REPEAT_START_ROCK {
            let position_in_sequence = (num_rocks - REPEAT_START_ROCK) % REPEAT_LENGTH;
            if i_rock == REPEAT_START_ROCK {
                repeat_start_height = grid.len() as usize + cleared_row_offset;
            } else if i_rock == REPEAT_START_ROCK + REPEAT_LENGTH {
                repeat_end_height = grid.len() as usize + cleared_row_offset;
            } else if i_rock == REPEAT_START_ROCK + REPEAT_LENGTH + position_in_sequence {
                break;
            }
            if i_rock > REPEAT_START_ROCK && (i_rock - REPEAT_START_ROCK) % REPEAT_LENGTH == 0 {
                let rps = i_rock as f64 / start.elapsed().as_secs_f64();
                if rps > 0.0 {
                    let rocks_remaining = num_rocks - i_rock;
                    let time_remaining_s: usize = rocks_remaining / rps as usize;
                    let time_remaining_m: usize = time_remaining_s / 60;
                    let time_remaining_h: f64 = time_remaining_m as f64 / 60.0;
                    let time_remaining_d: f64 = time_remaining_h as f64 / 24.0;
                    let avg_repeat_length =
                        (grid.len() as usize + cleared_row_offset) % REPEAT_LENGTH;
                    if repeat_lengths.contains(&avg_repeat_length) {
                        println!(
                            "Rock num: {} ({:.2} rocks/s), {}s/{}m/{:.2}h/{:.2}d remaining; height: {}",
                            i_rock,
                            rps,
                            time_remaining_s,
                            time_remaining_m,
                            time_remaining_h,
                            time_remaining_d,
                            avg_repeat_length);
                    } else {
                        repeat_lengths.insert(avg_repeat_length);
                    }
                }
            }
        }
    }

    let position_in_sequence_height = grid.len() as usize + cleared_row_offset - repeat_end_height;
    let repeat_height = repeat_end_height - repeat_start_height;
    if num_rocks > REPEAT_START_ROCK + REPEAT_LENGTH {
        let position_in_sequence = (num_rocks - REPEAT_START_ROCK) % REPEAT_LENGTH;
        let num_repeats = (num_rocks - position_in_sequence - REPEAT_START_ROCK) / REPEAT_LENGTH;
        assert_eq!(
            num_rocks,
            position_in_sequence + REPEAT_START_ROCK + num_repeats * REPEAT_LENGTH
        );
        position_in_sequence_height + repeat_end_height + repeat_height * (num_repeats - 1) - 1
    } else {
        grid.len() as usize + cleared_row_offset
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return println!(
            "Usage: {} [jet file] [rocks file]",
            env::args().next().expect("Couldn't get executable name")
        );
    }
    let jet_input_name: &String = &args[1];
    let jet_input_f = File::open(jet_input_name).expect("Couldn't open input file");
    let jet_input: String = io::BufReader::new(jet_input_f)
        .lines()
        .next()
        .expect("Couldn't read input line")
        .expect("Couldn't read input line")
        .trim()
        .to_string();

    let rocks_name: &String = &args[2];
    let rocks_f = File::open(rocks_name).expect("Couldn't open input file");
    let rocks_lines: Vec<String> = io::BufReader::new(rocks_f)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let rock_sequence = parse_rock_shapes(&rocks_lines);
    let jet_sequence = parse_jet_input(&jet_input);

    if DEBUG || VERBOSE {
        println!(
            "Part 1: {}",
            simulate_rocks(&rock_sequence, &jet_sequence, 20)
        );
    } else {
        println!(
            "Part 1: {}",
            simulate_rocks(&rock_sequence, &jet_sequence, 2022)
        );
    }

    println!(
        "Part 2: {}",
        simulate_rocks(&rock_sequence, &jet_sequence, 1_000_000_000_000)
    );
}
