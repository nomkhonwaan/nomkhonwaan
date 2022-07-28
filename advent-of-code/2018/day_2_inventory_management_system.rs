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

    if let Some((a, b)) = find_two_correct_box_ids(&list_of_box_ids) {
        println!("second part answer is: {}", String::from_iter(intersect(&a, &b)));
    }
}

/// Represent each line of the ID
#[derive(Debug, Default)]
struct BoxId {
    // for comparing with each other BoxId
    chars: Vec<char>,
    sorted_chars: Vec<char>,

}

impl BoxId {
    fn from_string(s: String) -> Self {
        let mut c: Vec<char> = s.chars().collect();
        // sort the given id ascending
        c.sort();
        BoxId {
            chars: s.chars().collect(),
            sorted_chars: c,
        }
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

fn find_two_correct_box_ids(list_of_box_ids: &Vec<BoxId>) -> Option<(&BoxId, &BoxId)> {
    for (i, a) in list_of_box_ids.iter().enumerate() {
        for b in list_of_box_ids.iter().skip(i + 1) {
            if differ(a, b) == 1 {
                return Some((a, b));
            }
        }
    }
    None
}

/// Return number of different characters between box id `a` and `b`.
fn differ(a: &BoxId, b: &BoxId) -> usize {
    let mut diff: usize = 0;
    for (i, c) in a.chars.iter().enumerate() {
        if *c != b.chars[i] {
            diff += 1;
        }
    }
    diff
}

/// Return a set of characters are in both of box id `a` and `b`.
fn intersect(a: &BoxId, b: &BoxId) -> Vec<char> {
    let mut chars: Vec<char> = vec![];
    for (i, c) in a.chars.iter().enumerate() {
        if *c == b.chars[i] {
            chars.push(*c);
        }
    }
    chars
}