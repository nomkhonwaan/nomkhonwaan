use regex::{Captures, Regex};
use std::collections::VecDeque;
use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    //     [G] [R]                 [P]
    //     [H] [W]     [T] [P]     [H]
    //     [F] [T] [P] [B] [D]     [N]
    // [L] [T] [M] [Q] [L] [C]     [Z]
    // [C] [C] [N] [V] [S] [H]     [V] [G]
    // [G] [L] [F] [D] [M] [V] [T] [J] [H]
    // [M] [D] [J] [F] [F] [N] [C] [S] [F]
    // [Q] [R] [V] [J] [N] [R] [H] [G] [Z]
    //  1   2   3   4   5   6   7   8   9
    let mut stacks_of_crate: Vec<Vec<char>> = vec![
        vec!['Q', 'M', 'G', 'C', 'L'],
        vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'],
        vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'],
        vec!['J', 'F', 'D', 'V', 'Q', 'P'],
        vec!['N', 'F', 'M', 'S', 'L', 'B', 'T'],
        vec!['R', 'N', 'V', 'H', 'C', 'D', 'P'],
        vec!['H', 'C', 'T'],
        vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'],
        vec!['Z', 'F', 'H', 'G'],
    ];
    let mut stacks_of_crate_cloned = stacks_of_crate.clone();

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                if re.is_match(&line) {
                    let matches: Captures = re.captures(&line).unwrap();
                    let n: u32 = matches.get(1).unwrap().as_str().parse().unwrap();
                    let from: usize = matches.get(2).unwrap().as_str().parse().unwrap();
                    let to: usize = matches.get(3).unwrap().as_str().parse().unwrap();

                    move_by_giant_cargo_crane(&mut stacks_of_crate, n, from - 1, to - 1);
                    move_by_crate_mover_9001(&mut stacks_of_crate_cloned, n, from - 1, to - 1)
                }
            }
        }
    }

    println!("first part answer is: {}", get_top_of_stack(&stacks_of_crate));
    println!("second part answer is: {}", get_top_of_stack(&stacks_of_crate_cloned));
}

fn get_top_of_stack(stacks_of_crate: &Vec<Vec<char>>) -> String {
    let mut top_of_stack = String::from("");
    for s in stacks_of_crate.iter() {
        if let Some(c) = s.last() {
            top_of_stack.push(*c);
        }
    }
    top_of_stack
}

fn move_by_giant_cargo_crane(stacks_of_crate: &mut Vec<Vec<char>>, n: u32, from: usize, to: usize) {
    for _ in 0..n {
        if let Some(c) = stacks_of_crate[from].pop() {
            stacks_of_crate[to].push(c);
        }
    }
}

fn move_by_crate_mover_9001(stacks_of_crate: &mut Vec<Vec<char>>, n: u32, from: usize, to: usize) {
    let mut s: VecDeque<char> = VecDeque::new();

    for _ in 0..n {
        if let Some(c) = stacks_of_crate[from].pop() {
            s.push_front(c);
        }
    }

    stacks_of_crate[to].append(&mut Vec::from(s));
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
