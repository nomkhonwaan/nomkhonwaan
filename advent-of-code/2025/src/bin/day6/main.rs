use advent_of_code_2025::read_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut problems: Vec<Problem> = vec![];

    let content = read_file(&args[1]).unwrap();
    let last_line = content.lines().last().unwrap();

    for (i, s) in last_line.chars().enumerate() {
        if s == '+' || s == '-' || s == '*' || s == '/' {
            if problems.is_empty() {
                problems.push(Problem {
                    start_index: i,
                    end_index: 0,
                    numbers: vec![],
                    numbers_str: vec![],
                    operation: s.to_string(),
                });
            } else {
                problems.last_mut().unwrap().end_index = i - 1;
                problems.push(Problem {
                    start_index: i,
                    end_index: last_line.len(),
                    numbers: vec![],
                    numbers_str: vec![],
                    operation: s.to_string(),
                });
            }
        }
    }

    let lines: Vec<&str> = content.lines().collect();
    for line in &lines[..lines.len().saturating_sub(1)] {
        for problem in problems.iter_mut() {
            let part = &line[problem.start_index..problem.end_index];
            let number = part.trim().parse::<i64>().unwrap();
            problem.numbers.push(number);
            problem.numbers_str.push(part.to_string());
        }
    }

    println!("First part answer: {}", cal_first_part_answer(&problems));
    println!("Second part answer: {}", cal_second_part_answer(&problems));
}

fn cal_first_part_answer(problems: &Vec<Problem>) -> i64 {
    problems.iter().map(|p| p.solve()).sum()
}

fn cal_second_part_answer(problems: &Vec<Problem>) -> i64 {
    let mut problems = problems
        .iter()
        .map(|p| Problem {
            numbers: p.right_to_left(),
            ..p.clone()
        })
        .collect::<Vec<Problem>>();
    problems.reverse();
    problems.iter().map(|p| p.solve()).sum()
}

#[derive(Clone, Debug, Default)]
struct Problem {
    start_index: usize,
    end_index: usize,

    numbers: Vec<i64>,
    numbers_str: Vec<String>,

    operation: String,
}

impl Problem {
    fn solve(&self) -> i64 {
        match self.operation.as_str() {
            "+" => self.numbers.iter().sum(),
            "-" => self.numbers[1..]
                .iter()
                .fold(self.numbers[0], |acc, &x| acc - x),
            "*" => self.numbers.iter().product(),
            "/" => self.numbers[1..]
                .iter()
                .fold(self.numbers[0], |acc, &x| acc / x),
            _ => panic!("Unknown operation: {}", self.operation),
        }
    }

    // Return numbers parsed from right to left
    // Example
    // "123", " 45", "  6" -> vec![356, 24, 1]
    // "64 ", "23 ", "314" -> vec![4, 431, 623]
    fn right_to_left(&self) -> Vec<i64> {
        let max_len = self.numbers_str.iter().map(|s| s.len()).max().unwrap_or(0);
        (0..max_len)
            .filter_map(|pos| {
                let digits: String = self
                    .numbers_str
                    .iter()
                    .filter_map(|s| {
                        let idx = s.len().checked_sub(pos + 1)?;
                        s.chars().nth(idx).filter(|&c| c != ' ')
                    })
                    .collect();
                digits.parse::<i64>().ok()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_to_left() {
        let problem = Problem {
            numbers_str: vec!["123".to_string(), " 45".to_string(), "  6".to_string()],
            ..Default::default()
        };
        assert_eq!(vec![356, 24, 1], problem.right_to_left());

        let problem2 = Problem {
            numbers_str: vec!["64 ".to_string(), "23 ".to_string(), "314".to_string()],
            ..Default::default()
        };
        assert_eq!(vec![4, 431, 623], problem2.right_to_left());
    }
}
