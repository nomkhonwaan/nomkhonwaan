use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut equations: Vec<Equation> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                equations.push(Equation::from_str(&line));
            }
        }
    }

    println!(
        "First part answer is: {}",
        cal_first_part_answer(&equations)
    );
    println!(
        "Second part answer is: {}",
        cal_second_part_answer(&equations)
    );
}

fn cal_first_part_answer(equations: &[Equation]) -> u64 {
    let mut calibration_result = 0;
    for equation in equations {
        if equation.is_valid_equation() {
            calibration_result += equation.test_value;
        }
    }
    calibration_result
}

fn cal_second_part_answer(equations: &[Equation]) -> u64 {
    let mut calibration_result = 0;
    for equation in equations {
        if equation.is_valid_equation_with_combinations() {
            calibration_result += equation.test_value;
        }
    }
    calibration_result
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split(':').collect();
        let test_value: u64 = parts[0].trim().parse().unwrap();
        let numbers: Vec<u64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Equation {
            test_value,
            numbers,
        }
    }

    fn is_valid_equation(&self) -> bool {
        if self.numbers.is_empty() {
            return false;
        }
        if self.numbers.len() == 1 {
            return self.numbers[0] == self.test_value;
        }

        // Try all possible combinations of operators between numbers
        // For n numbers, we need n-1 operators
        let num_operators = self.numbers.len() - 1;

        // Generate all possible combinations of operators
        // Each position can be either '+' or '*', so 2^(n-1) combinations
        for mask in 0..(1 << num_operators) {
            let mut result = self.numbers[0];

            for i in 0..num_operators {
                let operator = if (mask >> i) & 1 == 0 { '+' } else { '*' };
                let next_number = self.numbers[i + 1];

                match operator {
                    '+' => result += next_number,
                    '*' => result *= next_number,
                    _ => unreachable!(),
                }
            }

            if result == self.test_value {
                return true;
            }
        }

        false
    }

    fn is_valid_equation_with_combinations(&self) -> bool {
        if self.numbers.is_empty() {
            return false;
        }
        if self.numbers.len() == 1 {
            return self.numbers[0] == self.test_value;
        }

        // Try all possible combinations of operators between numbers
        // For n numbers, we need n-1 operators
        // Each position can be '+', '*', or '&' (concatenation), so 3^(n-1) combinations
        let num_operators = self.numbers.len() - 1;
        let total_combinations = 3_u32.pow(num_operators as u32);

        for combination in 0..total_combinations {
            let mut result = self.numbers[0];
            let mut temp_combination = combination;

            for i in 0..num_operators {
                // Extract operator for position i (0='+', 1='*', 2='&')
                let operator_code = temp_combination % 3;
                temp_combination /= 3;

                let next_number = self.numbers[i + 1];

                match operator_code {
                    0 => result += next_number, // '+' addition
                    1 => result *= next_number, // '*' multiplication
                    2 => {
                        // '&' concatenation
                        let combined_str = format!("{}{}", result, next_number);
                        result = combined_str.parse().unwrap();
                    }
                    _ => unreachable!(),
                }
            }

            if result == self.test_value {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_equation_with_combinations() {
        // Test the exact example from the user request
        // equation 3: 1, 2, 3
        // self.test_value = 33
        // self.numbers = vec![1, 2, 3];
        // 1 + 2 = 3, then 3 & 3 (concatenation) = 33
        let equation = Equation {
            test_value: 33,
            numbers: vec![1, 2, 3],
        };
        assert!(equation.is_valid_equation_with_combinations());

        // Test another example: 12 & 34 = 1234
        let equation2 = Equation {
            test_value: 1234,
            numbers: vec![12, 34],
        };
        assert!(equation2.is_valid_equation_with_combinations());

        // Test with multiplication and concatenation
        let equation3 = Equation {
            test_value: 246,
            numbers: vec![2, 4, 6],
        };
        // Should work with: 2 & 4 = 24, then 24 & 6 = 246
        assert!(equation3.is_valid_equation_with_combinations());

        // Test that should fail
        let equation4 = Equation {
            test_value: 99,
            numbers: vec![1, 2],
        };
        // 1 + 2 = 3, 1 * 2 = 2, 1 & 2 = 12 - none equals 99
        assert!(!equation4.is_valid_equation_with_combinations());
    }
}
