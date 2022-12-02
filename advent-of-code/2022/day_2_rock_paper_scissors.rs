use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut total_score = 0u32;
    let mut follow_strategy_total_score = 0u32;

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
               let chars = line.split(" ").collect::<Vec<&str>>();
               total_score += get_competition_score(chars[0], chars[1]) + get_shape_score(chars[1]);
               let opposite_shape = find_opposite_shape(chars[0], chars[1]);
               follow_strategy_total_score += get_competition_score(chars[0], opposite_shape) + get_shape_score(opposite_shape)
            }
        }
    }
    
    println!("first part answer is: {}", total_score);
    println!("second parse answer is: {}", follow_strategy_total_score);
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_shape_score(b: &str) -> u32 {
    let b = b.as_bytes()[0] as usize;
    vec![1, 2, 3][b - 88] // 88 = X
}

fn get_competition_score(a: &str, b: &str) -> u32 {
    let a = a.as_bytes()[0] - 65; // 65 = A
    let b = b.as_bytes()[0] - 88; // 88 = X

    if a == b {
        return 3;
    }
    if a == 0 && b == 1 {
        return 6;
    }
    if a == 1 && b == 2 {
        return 6;
    }
    if a == 2 && b == 0 {
        return 6;
    }

    0
}

fn find_opposite_shape<'a>(a: &'a str, strategy: &'a str) -> &'a str {
    let a = a.as_bytes()[0] as usize;

    match strategy {
        "X" => vec!["Z", "X", "Y"][a - 65], // lose
        "Y" => vec!["X", "Y", "Z"][a - 65], // draw
        "Z" => vec!["Y", "Z", "X"][a - 65], // win
        _ => panic!("invalid strategy")
    }
}
