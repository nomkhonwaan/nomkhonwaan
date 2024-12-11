use regex::Regex;
use std::{
    env, fs,
    io::{self, BufRead},
    path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let re = Regex::new(r"(mul|do|don't)\(((\d+),(\d+))?\)").unwrap();
    let mut first_part_answer: i32 = 0;
    let mut second_part_answer: i32 = 0;
    let mut should_mul = true;

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                for c in re.captures_iter(&line) {
                    match c.get(1).unwrap().as_str() {
                        "mul" => {
                            let x: i32 = c.get(3).unwrap().as_str().parse().unwrap();
                            let y: i32 = c.get(4).unwrap().as_str().parse().unwrap();

                            first_part_answer += x * y;

                            if should_mul {
                                second_part_answer += x * y;
                            }
                        }
                        "do" => {
                            should_mul = true;
                        }
                        "don't" => {
                            should_mul = false;
                        }
                        _ => (),
                    }
                }
            }
        }
    };

    println!("First part answer is: {}", first_part_answer); 
    println!("Second part answer is: {}", second_part_answer);
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
