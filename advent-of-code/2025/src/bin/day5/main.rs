use advent_of_code_2025::read_lines;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fresh_ingredient_ids: Vec<(usize, usize)> = vec![];
    let mut available_ingredient_ids: Vec<usize> = vec![];
    let mut found_blank_line = false;

    for line in read_lines(&args[1]).unwrap() {
        if line.is_empty() {
            found_blank_line = true;
            continue;
        }

        if found_blank_line {
            // Parse the available ingredient IDs
            available_ingredient_ids.push(line.parse::<usize>().unwrap());
        } else {
            // Parse the fresh ingredient IDs
            fresh_ingredient_ids.push(parse(&line));
        }
    }

    // Sort the fresh ingredient IDs by their starting ID
    fresh_ingredient_ids.sort();

    println!(
        "First part answer: {}",
        cal_first_part_answer(&fresh_ingredient_ids, &available_ingredient_ids)
    );
    println!(
        "Second part answer: {}",
        cal_second_part_answer(&fresh_ingredient_ids)
    );
}

fn cal_first_part_answer(
    fresh_ingredient_ids: &[(usize, usize)],
    available_ingredient_ids: &[usize],
) -> usize {
    let mut first_part_answer = 0;
    available_ingredient_ids.into_iter().for_each(|id| {
        for (start, end) in fresh_ingredient_ids {
            if id >= start && id <= end {
                first_part_answer += 1;
                break;
            }
        }
    });
    first_part_answer
}

fn cal_second_part_answer(fresh_ingredient_ids: &[(usize, usize)]) -> usize {
    let mut fresh_ingredient_ids = fresh_ingredient_ids.to_vec();
    fresh_ingredient_ids.sort();

    let mut total = 0_usize;
    let (mut cur_start, mut cur_end) = fresh_ingredient_ids[0];

    for (start, end) in fresh_ingredient_ids.into_iter().skip(1) {
        if start > cur_end + 1 {
            total += cur_end - cur_start + 1;
            cur_start = start;
            cur_end = end;
        } else {
            cur_end = cur_end.max(end);
        }
    }

    total + (cur_end - cur_start + 1)
}

fn parse(s: &str) -> (usize, usize) {
    let re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    if let Some(caps) = re.captures(s) {
        let start: usize = caps[1].parse().unwrap();
        let end: usize = caps[2].parse().unwrap();
        (start, end)
    } else {
        panic!("Invalid input format");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!((3, 5), parse("3-5"));
        assert_eq!((10, 14), parse("10-14"));
    }

    #[test]
    fn test_cal_first_part_answer() {
        let fresh_ingredient_ids = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let available_ingredient_ids = vec![1, 5, 8, 11, 17, 32];

        assert_eq!(
            3,
            cal_first_part_answer(&fresh_ingredient_ids, &available_ingredient_ids)
        );
    }

    #[test]
    fn test_cal_second_part_answer() {
        let fresh_ingredient_ids = vec![(3, 5), (10, 14), (16, 20), (12, 18)];

        assert_eq!(14, cal_second_part_answer(&fresh_ingredient_ids));
    }
}
