use advent_of_code_2025::read_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_file(&args[1]).unwrap();

    println!(
        "First part answer: {}",
        cal_first_part_answer(&parse(&input))
    );
    println!(
        "Second part answer: {}",
        cal_second_part_answer(&parse(&input))
    );
}

fn cal_first_part_answer(ids: &Vec<(usize, usize)>) -> usize {
    let mut first_part_answer = 0;

    for (start_id, end_id) in ids.iter() {
        for id in *start_id..=*end_id {
            if is_digit_repeated_twice(id) {
                first_part_answer += id;
            }
        }
    }

    first_part_answer
}

fn cal_second_part_answer(ids: &Vec<(usize, usize)>) -> usize {
    let mut second_part_answer = 0;

    for (start_id, end_id) in ids.iter() {
        for id in *start_id..=*end_id {
            if is_digit_repeated_at_least_twice(id) {
                second_part_answer += id;
            }
        }
    }

    second_part_answer
}

fn parse(line: &str) -> Vec<(usize, usize)> {
    line.split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s: &&str| {
            let ids: Vec<usize> = s
                .split("-")
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (ids[0], ids[1])
        })
        .collect::<Vec<(usize, usize)>>()
}

fn is_digit_repeated_twice(id: usize) -> bool {
    let s = id.to_string();
    if s.len() % 2 != 0 {
        return false;
    }

    let mid = s.len() / 2;
    let (first_half, second_half) = s.split_at(mid);
    first_half == second_half
}

fn is_digit_repeated_at_least_twice(id: usize) -> bool {
    let s = id.to_string();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=s.len() / 2 {
        let pattern = &s[0..pattern_len];

        // Check if the entire string is made up of this pattern repeated
        let mut is_repeated = true;
        for i in 0..s.len() {
            if s.chars().nth(i).unwrap() != pattern.chars().nth(i % pattern_len).unwrap() {
                is_repeated = false;
                break;
            }
        }

        if is_repeated && s.len() % pattern_len == 0 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![(10, 20), (30, 40), (50, 60)],
            parse("10-20,30-40,50-60")
        );
    }

    #[test]
    fn test_is_digit_repeated_twice() {
        assert_eq!(true, is_digit_repeated_twice(11));
        assert_eq!(true, is_digit_repeated_twice(22));
        assert_eq!(true, is_digit_repeated_twice(99));
        assert_eq!(true, is_digit_repeated_twice(1010));
        assert_eq!(true, is_digit_repeated_twice(1188511885));
        assert_eq!(true, is_digit_repeated_twice(222222));
        assert_eq!(true, is_digit_repeated_twice(446446));
        assert_eq!(true, is_digit_repeated_twice(38593859));
    }

    #[test]
    fn test_cal_first_part_answer() {
        assert_eq!(
            1227775554,
            cal_first_part_answer(&vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
            ])
        );
    }

    #[test]
    fn test_is_digit_repeated_at_least_twice() {
        assert_eq!(true, is_digit_repeated_at_least_twice(11));
        assert_eq!(true, is_digit_repeated_at_least_twice(22));
        assert_eq!(true, is_digit_repeated_at_least_twice(99));
        assert_eq!(true, is_digit_repeated_at_least_twice(111));
        assert_eq!(true, is_digit_repeated_at_least_twice(999));
        assert_eq!(true, is_digit_repeated_at_least_twice(1010));
        assert_eq!(true, is_digit_repeated_at_least_twice(1188511885));
        assert_eq!(true, is_digit_repeated_at_least_twice(222222));
        assert_eq!(true, is_digit_repeated_at_least_twice(446446));
        assert_eq!(true, is_digit_repeated_at_least_twice(565656));
        assert_eq!(true, is_digit_repeated_at_least_twice(38593859));
        assert_eq!(true, is_digit_repeated_at_least_twice(824824824));
        assert_eq!(true, is_digit_repeated_at_least_twice(2121212121));
    }

    #[test]
    fn test_cal_second_part_answer() {
        assert_eq!(
            4174379265,
            cal_second_part_answer(&vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
                (565653, 565659),
                (824824821, 824824827),
                (2121212118, 2121212124),
            ])
        );
    }
}
