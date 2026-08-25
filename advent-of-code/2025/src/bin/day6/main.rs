use advent_of_code_2025::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_lines(&args[1]).unwrap();

    let problems = parse_problems(&input);
    println!("First part answer: {}", cal_answer(&problems));

    let problems = parse_problems_right_to_left(&input);
    println!("Second part answer: {}", cal_answer(&problems));
}

fn parse_problems(input: &[String]) -> Vec<Problem> {
    let mut problems = Vec::new();

    for line in input {
        for (column, token) in line.split_whitespace().enumerate() {
            if column == problems.len() {
                problems.push(Problem::default());
            }
            problems[column].apply_token(token);
        }
    }

    problems
}

fn parse_problems_right_to_left(input: &[String]) -> Vec<Problem> {
    let Some(operator_line) = input.last() else {
        return Vec::new();
    };

    let mut problems: Vec<Problem> = operator_line
        .chars()
        .rev()
        .filter(|character| !character.is_whitespace())
        .map(Problem::with_operator)
        .collect();

    let grid: Vec<Vec<char>> = input[..input.len() - 1]
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect();

    let width = grid.iter().map(Vec::len).max().unwrap_or(0);
    let mut problem_index = 0;

    for column in 0..width {
        let token: String = grid
            .iter()
            .map(|row| row.get(column).copied().unwrap_or(' '))
            .collect();

        if token.trim().is_empty() {
            problem_index += 1;
        } else if let Some(problem) = problems.get_mut(problem_index) {
            problem.apply_token(token.trim());
        }
    }

    problems
}

fn cal_answer(problems: &[Problem]) -> u64 {
    problems.iter().map(|a| a.calculate()).sum()
}


#[derive(Debug, PartialEq)]
struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Problem {
    fn with_operator(operator: char) -> Self {
        let mut problem = Self::default();
        problem.apply_token(&operator.to_string());
        problem
    }

    fn apply_token(&mut self, token: &str) {
        match token {
            "+" => {
                self.operator = Operator::Plus;
            }
            "*" => {
                self.operator = Operator::Multiply;
            }
            _ => {
                self.numbers.push(token.parse().unwrap());
            }
        };
    }

    fn calculate(&self) -> u64 {
        self.numbers
            .iter()
            .copied()
            .reduce(|a, b| match self.operator {
                Operator::Plus => a + b,
                Operator::Multiply => a * b,
            })
            .unwrap()
    }
}

impl Default for Problem {
    fn default() -> Self {
        Self {
            numbers: vec![],
            operator: Operator::Plus,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Multiply,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_problems() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        let problems = parse_problems(&input);

        assert_eq!(
            problems,
            vec![
                Problem {
                    numbers: vec![123, 45, 6],
                    operator: Operator::Multiply
                },
                Problem {
                    numbers: vec![328, 64, 98],
                    operator: Operator::Plus
                },
                Problem {
                    numbers: vec![51, 387, 215],
                    operator: Operator::Multiply
                },
                Problem {
                    numbers: vec![64, 23, 314],
                    operator: Operator::Plus
                }
            ]
        );
    }

    #[test]
    fn test_parse_problems_right_to_left() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        let problems = parse_problems_right_to_left(&input);

        assert_eq!(
            problems,
            vec![
                Problem {
                    numbers: vec![4, 431, 623],
                    operator: Operator::Plus
                },
                Problem {
                    numbers: vec![175, 581, 32],
                    operator: Operator::Multiply
                },
                Problem {
                    numbers: vec![8, 248, 369],
                    operator: Operator::Plus
                },
                Problem {
                    numbers: vec![356, 24, 1],
                    operator: Operator::Multiply
                }
            ]
        )
    }

    #[test]
    fn test_cal_first_part_answer() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        let problems = parse_problems(&input);

        assert_eq!(cal_answer(&problems), 4277556);
    }

    #[test]
    fn test_cal_second_part_answer() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        let problems = parse_problems_right_to_left(&input);

        assert_eq!(cal_answer(&problems), 3263827);
    }
}
