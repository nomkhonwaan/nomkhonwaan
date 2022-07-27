use std::{env, fs, io, path};
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut list_of_box_ids: Vec<BoxId> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let box_id = BoxId::from_string(line);
                list_of_box_ids.push(box_id);
            }
        }
    }

    let contains_two: Vec<&BoxId> = list_of_box_ids.iter().filter(|box_id| box_id.contains_two_of_any_letter()).collect();
    let contains_three: Vec<&BoxId> = list_of_box_ids.iter().filter(|box_id| box_id.contains_three_of_any_letter()).collect();
    println!("first part answer is: {}", contains_two.len() * contains_three.len());
}

/// Represent each line of the ID
#[derive(Debug, Default)]
struct BoxId {
    sorted_chars: Vec<char>,
}

impl BoxId {
    fn from_string(s: String) -> Self {
        let mut c: Vec<char> = s.chars().collect();
        // sort the given id ascending
        c.sort();
        BoxId { sorted_chars: c }
    }

    fn contains_two_of_any_letter(&self) -> bool {
        let max = self.sorted_chars.len();

        for (i, c) in self.sorted_chars.iter().enumerate() {
            if i + 1 > max - 1 {
                break;
            }
            if *c != self.sorted_chars[i + 1] {
                continue;
            }

            // might contain three of any letter forward
            if i + 2 < max {
                if *c == self.sorted_chars[i + 2] {
                    continue;
                }
            }
            // might contain three of any letter backward
            if i > 0 {
                if *c == self.sorted_chars[i - 1] {
                    continue;
                }
            }

            return true;
        }

        false
    }

    fn contains_three_of_any_letter(&self) -> bool {
        let max = self.sorted_chars.len();

        for (i, c) in self.sorted_chars.iter().enumerate() {
            if i + 2 > max - 1 {
                break;
            }
            if *c == self.sorted_chars[i + 1] && *c == self.sorted_chars[i + 2] {
                return true;
            }
        }

        false
    }
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}