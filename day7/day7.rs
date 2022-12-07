use std::io;
use std::str::FromStr;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Clone,PartialEq,Eq,Debug)]
struct CommandLine {
    command: String,
    argument: String
}

#[derive(Clone,PartialEq,Eq,Debug)]
struct FileNode {
    name: String,
    size: usize,
}

#[derive(Clone,PartialEq,Eq,Debug)]
struct DirNode {
    name: String,
    children: Vec<Rc<RefCell<FSLine>>>,
}

#[derive(Clone,PartialEq,Eq,Debug)]
enum FSLine {
    Command(CommandLine),
    File(FileNode),
    Dir(DirNode),
}

impl FromStr for FSLine {
    type Err = ();
    fn from_str(input: &str) -> Result<FSLine, Self::Err> {
        let mut words = input.split_whitespace();
        match words.next().expect("Couldn't get first word") {
            "$" => Ok(FSLine::Command(CommandLine{
                command: words.next().expect("Couldn't get command type").to_string(),
                argument: words.next().unwrap_or("").to_string(),
            })),
            "dir" => Ok(FSLine::Dir(DirNode {
                name: words.next().expect("Couldn't get command type").to_string(),
                children: Vec::<_>::new(),
            })),
            size => Ok(FSLine::File(FileNode{
                size: size.parse::<usize>().expect("Couldn't parse size"),
                name: words.next().expect("Couldn't get command type").to_string(),
            })),
        }
    }
}

fn parse_lines(lines: &Vec<String>) -> Vec<FSLine> {
    lines.iter()
        .map(|line| FSLine::from_str(&line) )
        .filter_map(Result::ok).collect()
}

fn build_tree(mut lines: Vec<FSLine>) -> Rc<RefCell<FSLine>> {
    let mut iter = lines.iter_mut();

    if let FSLine::Command(first_line) = iter.next().expect("Couldn't get first line") {
        assert_eq!(first_line.command, "cd");
        assert_eq!(first_line.argument, "/");
    }
    else {
        panic!("First line should be directory");
    }

    let root_dir: Rc<RefCell<FSLine>> = Rc::new(RefCell::new(FSLine::Dir(DirNode {
        name: "/".to_string(),
        children: Vec::<_>::new(),
    })));
    let mut current_directory: Vec<Rc<RefCell<FSLine>>> = vec![root_dir.clone()];

    let mut ls_mode = false;
    while let Some(line) = iter.next() {
        match line {
            FSLine::Command(command) => {
                match command.command.as_str() {
                    "ls" => {
                        ls_mode = true;
                    },
                    "cd" => {
                        ls_mode = false;
                        if command.argument == ".." {
                            current_directory.pop();
                        }
                        else {
                            if let FSLine::Dir(ref mut cwd_as_dir) =
                                *(*current_directory.last()
                                    .expect("Couldn't get current directory for cd"))
                                    .clone()
                                    .borrow_mut() {
                                current_directory.push(cwd_as_dir.children.iter_mut()
                                    .find(|child| {
                                        if let FSLine::Dir(ref child_dir) = *(*child).borrow() {
                                            (*child_dir).name == command.argument
                                        }
                                        else {
                                            false
                                        }
                                    })
                                    .expect("Couldn't find dir to cd to")
                                    .clone());
                            }
                        }
                    },
                    _ => ()
                }
            },
            FSLine::Dir(_) => {
                assert_eq!(ls_mode, true);

                if let FSLine::Dir(ref mut cwd_as_dir) =
                    *(*current_directory.last()
                        .expect("Couldn't get current directory for cd"))
                        .clone()
                        .borrow_mut() {
                    cwd_as_dir.children.push(Rc::new(RefCell::new(line.to_owned())));
                }
            },
            FSLine::File(_) => {
                assert_eq!(ls_mode, true);

                if let FSLine::Dir(ref mut cwd_as_dir) =
                    *(*current_directory.last()
                        .expect("Couldn't get current directory for cd"))
                        .clone()
                        .borrow_mut() {
                    cwd_as_dir.children.push(Rc::new(RefCell::new(line.to_owned())));
                }
            },
        };
    };
    root_dir
}

// Returns (sizes of child directories, size of self)
fn size_of_dirs(dir: &DirNode) -> (Vec<usize>, usize) {
    let mut ret = Vec::<usize>::new();

    let dir_size = dir.children.iter()
        .map(|child| match &*child.borrow() {
            FSLine::Dir(dir) => {
                let (mut child_dir_sizes, child_size) = size_of_dirs(&dir);
                ret.append(&mut child_dir_sizes);
                child_size
            },
            FSLine::File(FileNode {name: _, size}) => *size,
            _ => 0,
        }).sum();

    ret.push(dir_size);

    (ret, dir_size)
}

fn size_of_dirs_with_names(dir: &DirNode, name: String) -> (HashMap::<String, usize>, usize) {
    let dir_name = (name + "/" + &dir.name).to_string();

    let mut ret = HashMap::<String, usize>::new();

    let dir_size = dir.children.iter()
        .map(|child| match &*child.borrow() {
            FSLine::Dir(dir) => {
                let (child_dir_sizes, child_size) = size_of_dirs_with_names(&dir, dir_name.to_string());
                ret.extend(child_dir_sizes);
                child_size
            },
            FSLine::File(FileNode {name: _, size}) => *size,
            _ => 0,
        }).sum();

    ret.insert(dir_name, dir_size);

    (ret, dir_size)
}

fn print_tree(node: &Rc<RefCell<FSLine>>, depth: usize) {
    print!("{: <1$}", "", depth * 2);
    match &*node.borrow() {
        FSLine::Dir(dir) => {
            println!("- {} (dir)", dir.name);
            dir.children.iter()
                .for_each(|child| print_tree(&child, depth + 1))
        },
        FSLine::File(FileNode {name, size}) => {
            println!("- {} (file, size={})", *name, *size);
        },
        _ => (),
    }
}

fn part1(sizes: &Vec<usize>, max_size: usize) -> usize {
    sizes.iter()
        .filter(|&size| *size <= max_size)
        .copied()
        .sum()
}

fn part2(sizes: &Vec<usize>, fs_size: usize, update_size: usize, fs_used: usize) -> usize {
    let unused_space = fs_size - fs_used;
    let space_needed = update_size - unused_space;
    sizes.iter()
        .filter(|&size| *size >= space_needed)
        .copied()
        .min()
        .expect("Couldn't find smallest dir that would free up enough space")
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();

    let parsed_lines = parse_lines(&lines);
    let tree = build_tree(parsed_lines.to_owned());

    const DEBUG: bool = false;

    if DEBUG {
        print_tree(&tree, 0);
    }

    if let FSLine::Dir(dir) = &*tree.borrow() {
        let (sizes, total_size) = size_of_dirs(&dir);

        println!("total size: {}", total_size);

        if DEBUG {
            let (sizes, _) = size_of_dirs_with_names(&dir, "".to_string());
            println!("{} total dirs:", sizes.len());
            for name in sizes.keys().sorted() {
                println!("{}: {}", name, sizes[name]);
            }
        }

        println!("Part 1: {}", part1(&sizes, 100_000));
        println!("Part 2: {}", part2(&sizes, 70_000_000, 30_000_000, total_size));
    };
}
