use std::{collections::HashMap, env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut sum_of_the_priorities = 0u32;
    let mut sum_of_the_priorities_between_group = 0u32;
    let mut i = 0usize;

    if let Ok(lines) = read_lines(input) {
        let mut group: Vec<String> = vec![];
        for line in lines {
            if let Ok(line) = line {
                sum_of_the_priorities += get_priority(find_shared_item(&line) as u32);

                // for finding shared item between group of three
                i += 1;
                group.push(line);

                if i % 3 == 0 {
                    sum_of_the_priorities_between_group +=
                        get_priority(find_shared_item_between_group(group) as u32);

                    // clear previous group compartments
                    group = vec![];
                }
            }
        }
    }

    println!("first part answer is: {}", sum_of_the_priorities);
    println!(
        "second part answer is: {}",
        sum_of_the_priorities_between_group
    );
}

fn find_shared_item(compartments: &str) -> char {
    let n = compartments.len() / 2;
    let first_compartment: String = compartments.chars().take(n).collect();
    let second_compartment: String = compartments.chars().skip(n).collect();
    let mut compartments: HashMap<char, u32> = HashMap::new();

    for k in first_compartment.chars() {
        compartments.entry(k).or_insert(1);
    }

    for k in second_compartment.chars() {
        if compartments.contains_key(&k) {
            return k;
        }
    }

    ' '
}

fn find_shared_item_between_group(groups: Vec<String>) -> char {
    let mut compartments: HashMap<char, u32> = HashMap::new();

    for k in groups[0].chars() {
        compartments.entry(k).or_insert(1);
    }

    for k in groups[1].chars() {
        compartments.entry(k).and_modify(|v| *v = 2);
    }

    for k in groups[2].chars() {
        if let Some(v) = compartments.get(&k) {
            if *v == 2 {
                return k;
            }
        }
    }

    ' '
}

fn get_priority(ascii: u32) -> u32 {
    // lowercase item priorities 1 through 26
    if ascii >= 97 {
        return ascii - 97 + 1;
    }
    // uppercase item priorities 27 through 52
    return ascii - 65 + 27;
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
