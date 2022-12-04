use std::{env, fs, io, io::BufRead, path, str::FromStr, string::ParseError};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut fully_contains = 0u32;
    let mut overlap_at_all = 0u32;

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let line: Vec<&str> = line.split(",").collect();
                let first_pair: Pair = line[0].parse().unwrap();
                let second_pair: Pair = line[1].parse().unwrap();

                if contains(&first_pair, &second_pair) || contains(&second_pair, &first_pair) {
                    fully_contains += 1;
                }

                if overlaps(&first_pair, &second_pair) || overlaps(&second_pair, &first_pair) {
                    overlap_at_all += 1;
                }
            }
        }
    }

    println!("first part answer is: {}", fully_contains);
    println!("second part answer is: {}", overlap_at_all);
}

#[derive(Debug, Default)]
struct Pair {
    x: u32,
    y: u32,
}

impl FromStr for Pair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split("-").collect();
        Ok(Pair {
            x: s[0].parse().unwrap(),
            y: s[1].parse().unwrap(),
        })
    }
}

fn contains(a: &Pair, b: &Pair) -> bool {
    return a.x <= b.x && a.y >= b.y;
}

fn overlaps(a: &Pair, b: &Pair) -> bool {
    return a.x <= b.x && a.y >= b.x
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
