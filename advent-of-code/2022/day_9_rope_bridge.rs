use std::{env, fs, io, io::BufRead, path};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut head1: Vec<(isize, isize)> = vec![(0, 0)];
    let mut head2: Vec<(isize, isize)> = vec![(0, 0)];
    let mut tail: Vec<(isize, isize)> = vec![(0, 0)];
    let mut nine_tails: Vec<Vec<(isize, isize)>> = vec![vec![(0, 0)]; 9];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let line: Vec<&str> = line.split_whitespace().collect();
                let n: isize = line[1].parse().unwrap();
                for _ in 0..n {
                    move_head_with_tail(&mut head1, &mut tail, line[0]);
                    move_head_with_nine_tails(&mut head2, &mut nine_tails, line[0]);
                }
            }
        }
    }

    let tail = tail.into_iter()
        .fold(HashMap::new(), |mut result: HashMap<(isize, isize), bool>, v| {
            result.entry(v).or_insert(true);
            result
        });
    println!("first part answer is: {}", tail.values().len());

    let ninth_tail = nine_tails[8].clone().into_iter()
        .fold(HashMap::new(), |mut result: HashMap<(isize, isize), bool>, v| {
            result.entry(v).or_insert(true);
            result
        });
    println!("second part answer is: {}", ninth_tail.values().len());
}

fn move_head_with_tail(head: &mut Vec<(isize, isize)>, tail: &mut Vec<(isize, isize)>, direction: &str) {
    let mut h = *head.last().unwrap();
    let mut t = *tail.last().unwrap();

    h = moves(direction, h);
    head.push(h);

    let adj = find_adjacent(h, t);

    if adj.0.abs() > 1 || adj.1.abs() > 1 {
        if h.0 != t.0 {
            if h.0 > t.0 {
                t.0 += 1;
            } else {
                t.0 -= 1;
            }
        }
        if h.1 != t.1 {
            if h.1 > t.1 {
                t.1 += 1;
            } else {
                t.1 -= 1;
            }
        }
        tail.push(t);
    }
}

fn move_head_with_nine_tails(head: &mut Vec<(isize, isize)>, tails: &mut Vec<Vec<(isize, isize)>>, direction: &str) {
    let mut h = *head.last().unwrap();

    h = moves(direction, h);
    head.push(h);

    for i in 0..tails.len() {
        let mut t = *tails[i].last().unwrap();
        if i != 0 {
            h = *tails[i - 1].last().unwrap();
        }

        let adj = find_adjacent(h, t);

        if adj.0.abs() > 1 || adj.1.abs() > 1 {
            if h.0 != t.0 {
                if h.0 > t.0 {
                    t.0 += 1;
                } else {
                    t.0 -= 1;
                }
            }
            if h.1 != t.1 {
                if h.1 > t.1 {
                    t.1 += 1;
                } else {
                    t.1 -= 1;
                }
            }
            tails[i].push(t);
        }
    }
}

fn moves(direction: &str, pos: (isize, isize)) -> (isize, isize) {
    let mut pos = pos;
    match direction {
        "R" => pos.1 += 1,
        "U" => pos.0 += 1,
        "L" => pos.1 -= 1,
        "D" => pos.0 -= 1,
        _ => {}
    }
    pos
}

fn find_adjacent(h: (isize, isize), t: (isize, isize)) -> (isize, isize) {
    ((h.0 as isize - t.0 as isize),
     (h.1 as isize - t.1 as isize))
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
