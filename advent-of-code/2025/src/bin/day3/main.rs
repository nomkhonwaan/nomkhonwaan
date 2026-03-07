use advent_of_code_2025::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: Vec<Vec<u64>> = vec![];

    for line in read_lines(&args[1]).unwrap() {
        input.push(parse(&line));
    }

    println!("First part answer: {}", cal_first_part_answer(&input));
    println!("Second part answer: {}", cal_second_part_answer(&input));
}

fn cal_first_part_answer(input: &Vec<Vec<u64>>) -> u64 {
    let mut first_part_answer = 0;

    for batt in input.iter() {
        let joltage = find_joltage(batt, 2);
        first_part_answer += joltage;
    }

    first_part_answer
}

fn cal_second_part_answer(input: &Vec<Vec<u64>>) -> u64 {
    let mut second_part_answer = 0;

    for batt in input.iter() {
        let joltage = find_joltage(batt, 12);
        second_part_answer += joltage;
    }

    second_part_answer
}

fn parse(line: &str) -> Vec<u64> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

fn find_the_largest_number_with_index(batteries: &Vec<u64>, n: usize) -> (usize, u64) {
    let mut index = 0usize;
    let mut highest_value = 0u64;

    for (i, &battery) in batteries.iter().enumerate() {
        if i > batteries.len() - n {
            break;
        }

        if battery > highest_value {
            highest_value = battery;
            index = i;
        }
    }

    (index, highest_value)
}

fn find_joltage(batteries: &Vec<u64>, n: usize) -> u64 {
    let (index, starting_value) = find_the_largest_number_with_index(batteries, n);

    // Find possible number from starting index
    let mut joltage = starting_value;
    let mut current_index = index + 1;
    let mut remaining = n - 1;

    while remaining > 0 && current_index < batteries.len() {
        // Find the maximum value within the valid window
        // We can search from current_index to (batteries.len() - remaining)
        // to ensure we have enough elements left for remaining digits
        let max_search_index = batteries.len() - remaining;
        let mut max_value = 0u64;
        let mut max_idx = current_index;

        for i in current_index..=max_search_index {
            if batteries[i] > max_value {
                max_value = batteries[i];
                max_idx = i;
            }
        }

        joltage = joltage * 10 + max_value;
        current_index = max_idx + 1;
        remaining -= 1;
    }

    joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_largest_number_with_index() {
        assert_eq!((0, 1), find_the_largest_number_with_index(&vec![1, 2], 2));
        assert_eq!(
            (1, 2),
            find_the_largest_number_with_index(&vec![1, 2, 3], 2)
        );
        assert_eq!(
            (0, 1),
            find_the_largest_number_with_index(&vec![1, 2, 3], 3)
        );
        assert_eq!(
            (1, 2),
            find_the_largest_number_with_index(&vec![1, 2, 3, 4], 3)
        );
    }

    #[test]
    fn test_find_joltage() {
        assert_eq!(
            987654321111,
            find_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12)
        );
        assert_eq!(
            811111111119,
            find_joltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12)
        );
        assert_eq!(
            888911112111,
            find_joltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12)
        );
    }
}
