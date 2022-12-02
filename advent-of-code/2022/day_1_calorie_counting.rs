use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut elves: Vec<u32> = vec![];

    if let Ok(lines) = read_lines(input) {
        let mut carrying_calories = 0u32;
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    elves.push(carrying_calories);
                    carrying_calories = 0;
                    continue;
                }

                carrying_calories += line.parse::<u32>().expect("invalid number");
            }
        }
    }
    
    // descending order
    elves.sort_by(|a, b| b.cmp(a));

    println!("first part answer is: {}", elves[0]);
    println!("second part answer: {}", elves[0] + elves[1] + elves[2]);
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
