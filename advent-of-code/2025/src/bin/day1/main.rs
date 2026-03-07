use advent_of_code_2025::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let current: i32 = 50;
    let mut rotations: Vec<(i32, i32)> = vec![];

    read_lines(input).unwrap().iter().for_each(|line| {
        let (command, value) = parse(line);
        rotations.push((command, value));
    });

    println!(
        "First part answer: {}",
        cal_first_part_answer(&rotations, current)
    );
    println!(
        "Second part answer: {}",
        cal_second_part_answer(&rotations, current)
    );
}

fn cal_first_part_answer(rotations: &Vec<(i32, i32)>, current: i32) -> i32 {
    let mut current = current;
    let mut first_part_answer = 0;

    for (command, value) in rotations.iter() {
        let value = value % 100;

        current += command * value;
        if current > 99 {
            current -= 100;
        } else if current < 0 {
            current += 100;
        }

        if current == 0 {
            first_part_answer += 1;
        }
    }

    first_part_answer
}

fn cal_second_part_answer(rotations: &Vec<(i32, i32)>, current: i32) -> u32 {
    let mut current = current;
    let mut second_part_answer = 0;

    for (command, value) in rotations.iter() {
        for _ in 1..=*value {
            current += command * 1;

            if current > 99 {
                current -= 100;
            } else if current < 0 {
                current += 100;
            }

            if current == 0 {
                second_part_answer += 1;
            }
        }
    }

    second_part_answer
}

fn parse(line: &String) -> (i32, i32) {
    let command = match &line[0..1] {
        "R" => 1,
        "L" => -1,
        _ => panic!("Unknown command"),
    };
    let value: i32 = line[1..].parse().unwrap();
    (command, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cal_second_part_answer() {
        assert_eq!(1, cal_second_part_answer(&vec![(-1, 50), (1, 50)], 50));
        assert_eq!(1, cal_second_part_answer(&vec![(-1, 50), (-1, 50)], 50));
        assert_eq!(1, cal_second_part_answer(&vec![(1, 50), (-1, 50)], 50));
        assert_eq!(1, cal_second_part_answer(&vec![(1, 50), (1, 50)], 50));
        assert_eq!(2, cal_second_part_answer(&vec![(-1, 150), (-1, 50)], 50));
        assert_eq!(2, cal_second_part_answer(&vec![(-1, 150), (1, 50)], 50));
        assert_eq!(2, cal_second_part_answer(&vec![(1, 150), (-1, 50)], 50));
        assert_eq!(2, cal_second_part_answer(&vec![(1, 150), (1, 50)], 50));
        assert_eq!(
            6,
            cal_second_part_answer(
                &vec![
                    (-1, 68),
                    (-1, 30),
                    (1, 48),
                    (-1, 5),
                    (1, 60),
                    (-1, 55),
                    (-1, 1),
                    (-1, 99),
                    (1, 14),
                    (-1, 82)
                ],
                50
            )
        );
    }
}
