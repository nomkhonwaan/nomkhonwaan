use regex::Regex;
use std::{collections::HashMap, env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let re = Regex::new(r"(\d+)\s+(\d+)$").unwrap();

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let matches = re.captures(&line).unwrap();

                left.push(matches.get(1).unwrap().as_str().parse().unwrap());
                right.push(matches.get(2).unwrap().as_str().parse().unwrap());
            }
        }
    }

    left.sort();
    right.sort();

    let count = count_duplicates(&right);

    let mut first_part_answer = 0i32;
    let mut second_part_answer = 0i32;

    for (i, l) in left.iter().enumerate() {
        first_part_answer += (right[i] - l).abs();
        second_part_answer += match count.get(&l) {
            Some(v) => l * v,
            None => l * 0,
        };
    }

    println!("First part answer is: {}", first_part_answer);
    println!("Second part answer is: {}", second_part_answer);
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_duplicates(v: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();

    for n in v {
        let counter = counts.entry(*n).or_insert(0);
        *counter += 1;
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_duplicates() {
        let v = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        let counts = count_duplicates(&v);

        assert_eq!(counts.get(&1), Some(&2));
        assert_eq!(counts.get(&2), Some(&2));
        assert_eq!(counts.get(&3), Some(&2));
        assert_eq!(counts.get(&4), Some(&2));
        assert_eq!(counts.get(&5), Some(&2));
    }
}
