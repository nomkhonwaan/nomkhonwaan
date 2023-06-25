use futures::future::join_all;
use std::{env, fs, io, io::Read, path};
use tokio::task::spawn;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let polymer = read_file(input).unwrap();

    println!("first part answer is: {}", reduce(&polymer).len());

    let instances = "abcdefghijklmnopqrstuvwxyz";
    let mut tasks = vec![];

    for instance in instances.chars().collect::<Vec<char>>() {
        let task = spawn(produce(polymer.clone(), instance));
        tasks.push(task);
    }

    let shortest_polymer_len = join_all(tasks)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<usize>>()
        .into_iter()
        .min();

    println!(
        "second part answer is: {}",
        shortest_polymer_len.unwrap_or(polymer.len())
    );
}

async fn produce(mut polymer: String, instance: char) -> usize {
    polymer = polymer.replace(instance, "");
    polymer = polymer.replace(instance.to_ascii_uppercase(), "");

    reduce(&polymer).len()
}

fn reduce(polymer: &String) -> String {
    let mut units: Vec<char> = polymer.chars().collect();

    loop {
        let mut reacted = false;

        for i in 0..units.len() - 1 {
            if react(&units[i], &units[i + 1]) {
                // Remove the current and next item from the units vector.
                units.remove(i);
                // The above remove function makes the next index changed,
                // so the next index will be i instead of i + 1.
                units.remove(i);

                reacted = true;
                break;
            }
        }

        // Loop until no reaction, then break.
        if !reacted {
            break;
        }
    }

    units.iter().collect()
}

fn react(n: &char, m: &char) -> bool {
    // Nothing happens when they are the same type and their polarities match.
    // Example: aa, AA, CC
    if n == m {
        return false;
    }
    // At this statement, they might be the same type or not,
    // But their polarities are not the same for sure.
    return n.to_ascii_lowercase() == m.to_ascii_lowercase();
}

fn read_file<P: AsRef<path::Path>>(path: P) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
