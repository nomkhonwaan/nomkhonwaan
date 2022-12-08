use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut characters_before_start_of_packet_marker = 0usize;
	let mut characters_before_start_of_message_marker = 0usize;

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                characters_before_start_of_packet_marker += find_start_of_packet_marker_position(&line);
				characters_before_start_of_message_marker += find_start_of_message_marker_position(&line);
            }
        }
    }

    println!("first part answer is: {}", characters_before_start_of_packet_marker);
	println!("second part answer is: {}", characters_before_start_of_message_marker);
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_start_of_packet_marker_position(s: &str) -> usize {
    for i in 0..=s.len() - 4 {
        let four_characters = s.chars().skip(i).take(4).collect::<String>();
        if !is_duplicated(&four_characters) {
            return i + 4;
        }
    }
    0
}

fn find_start_of_message_marker_position(s: &str) -> usize {
    for i in 0..=s.len() - 14 {
        let fourteenth_characters = s.chars().skip(i).take(14).collect::<String>();
        if !is_duplicated(&fourteenth_characters) {
            return i + 14;
        }
    }
    0
}

fn is_duplicated(s: &str) -> bool {
    let mut m: Vec<u8> = vec![0; 26];
    for c in s.chars() {
        m[(c as usize) - 97] += 1;
        if m[c as usize - 97] > 1 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start_of_packet_marker_position() {
        assert_eq!(
            7,
            find_start_of_packet_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
    }

    #[test]
    fn test_find_start_of_message_marker_position() {
        assert_eq!(
            19,
            find_start_of_message_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
    }

    #[test]
    fn test_is_duplicated() {
        assert_eq!(true, is_duplicated("mjqj"));
        assert_eq!(false, is_duplicated("jpqm"));
    }
}
