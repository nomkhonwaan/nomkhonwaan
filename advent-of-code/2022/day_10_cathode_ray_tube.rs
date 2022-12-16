use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut x: Vec<i32> = vec![1];
    let mut sum_of_signal_strength = 0i32;
    let mut screen: Vec<char> = vec!['.'; 240]; // could replace '.' with ' ' for better reading

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                x.push(*x.get(x.len() - 1).unwrap());

                let chars: Vec<&str> = line.split_whitespace().collect();
                if let [_, v] = chars[..] {
                    x.push(*x.get(x.len() - 1).unwrap() + v.parse::<i32>().unwrap());
                }
            }
        }
    }

    for (i, v) in x.iter().enumerate() {
        if (v - (i as i32 % 40)).abs() <= 1 {
            screen[i] = '#';
        }

        if i > 0 {
            if (i as i32 - 20) % 40 == 0 {
                sum_of_signal_strength += i as i32 * x[i - 1];
            }
        }
    }

    println!("first part answer is: {}", sum_of_signal_strength);
    println!("second part answer is:");
    print_screen(&screen);
}

fn print_screen(screen: &Vec<char>) {
    for (i, v) in screen.iter().enumerate() {
        print!("{}", v);
        if (i as i32 + 1) % 40 == 0 {
            println!("");
        }
    }
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
