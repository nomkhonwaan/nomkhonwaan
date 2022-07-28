use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut first_part_answer = 0isize;
    let mut frequency: Vec<isize> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                // convert string to signed integer
                let current_frequency: isize = line.parse().expect("invalid number");
                first_part_answer += current_frequency;
                frequency.push(current_frequency);
            }
        }
    }

    println!("first part answer is: {}", first_part_answer);

    let second_part_answer = find_first_reaches_twice(&frequency);
    println!("second part answer is: {}", second_part_answer);
}

fn find_first_reaches_twice(frequency: &Vec<isize>) -> isize {
    let mut resulting_frequency = 0isize;
    let mut reaches: Vec<isize> = vec![0]; // first reaches is zero

    loop {
        for current_frequency in frequency {
            resulting_frequency += current_frequency;
            if reaches.contains(&resulting_frequency) {
                return resulting_frequency;
            }
            reaches.push(resulting_frequency);
        }
    }
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
