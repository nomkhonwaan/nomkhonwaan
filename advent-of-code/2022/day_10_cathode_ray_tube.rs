use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut x: Vec<i32> = vec![1];

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

	let mut sum_of_signal_strength = 0i32;

    for (i, _) in x.iter().enumerate() {
		if i == 0 {
			continue;
        }

        if (i as i32 - 20) % 40 == 0 {
			sum_of_signal_strength += (i as i32 * x[i - 1]);
        }
    }

    println!("first part answer is: {}", sum_of_signal_strength);
}


fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}