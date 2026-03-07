use advent_of_code_2025::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: Vec<Vec<char>> = vec![];

    for line in read_lines(&args[1]).unwrap() {
        input.push(parse(&line));
    }

    println!("First part answer: {}", cal_first_part_answer(&input));
    println!("Second part answer: {}", cal_second_part_answer(&input));
}

fn cal_first_part_answer(input: &Vec<Vec<char>>) -> u64 {
    let max_i = input[0].len();
    let max_j = input.len();
    let mut first_part_answer = 0;

    for j in 0..max_j {
        for i in 0..max_i {
            if input[j][i] == '@' {
                let adjacent_cells = find_adjacent_cells(i, j, &max_i, &max_j);

                let rolls_of_paper = adjacent_cells
                    .into_iter()
                    .filter(|(adj_i, adj_j)| input[*adj_j][*adj_i] == '@')
                    .collect::<Vec<(usize, usize)>>()
                    .len();

                if rolls_of_paper < 4 {
                    first_part_answer += 1;
                }
            }
        }
    }

    first_part_answer
}

fn cal_second_part_answer(input: &Vec<Vec<char>>) -> u64 {
    let mut input = input.clone();
    let max_i = input[0].len();
    let max_j = input.len();
    let mut second_part_answer = 0;

    loop {
        let mut visited: Vec<(usize, usize)> = vec![];

        for j in 0..max_j {
            for i in 0..max_i {
                if input[j][i] == '@' {
                    let adjacent_cells = find_adjacent_cells(i, j, &max_i, &max_j);

                    let rolls_of_paper = adjacent_cells
                        .into_iter()
                        .filter(|(adj_i, adj_j)| input[*adj_j][*adj_i] == '@')
                        .collect::<Vec<(usize, usize)>>()
                        .len();

                    if rolls_of_paper < 4 {
                        second_part_answer += 1;
                        visited.push((i, j));
                    }
                }
            }
        }

        if visited.is_empty() {
            break;
        }

        // Remove all visited
        visited.into_iter().for_each(|(i, j)| {
            input[j][i] = '.';
        });
    }

    second_part_answer
}

fn parse(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn find_adjacent_cells(i: usize, j: usize, max_i: &usize, max_j: &usize) -> Vec<(usize, usize)> {
    let mut adjacent_cells: Vec<(usize, usize)> = vec![];

    // Up
    if i >= 1 {
        adjacent_cells.push((i - 1, j));
    }
    // Down
    if i + 1 < *max_i {
        adjacent_cells.push((i + 1, j));
    }
    // Left
    if j >= 1 {
        adjacent_cells.push((i, j - 1));
    }
    // Right
    if j + 1 < *max_j {
        adjacent_cells.push((i, j + 1));
    }
    // Top-left
    if i >= 1 && j >= 1 {
        adjacent_cells.push((i - 1, j - 1));
    }
    // Top-right
    if i + 1 < *max_i && j >= 1 {
        adjacent_cells.push((i + 1, j - 1));
    }
    // Bottom-left
    if i >= 1 && j + 1 < *max_j {
        adjacent_cells.push((i - 1, j + 1));
    }
    // Bottom-right
    if i + 1 < *max_i && j + 1 < *max_j {
        adjacent_cells.push((i + 1, j + 1));
    }

    adjacent_cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            vec!['.', '.', '@', '@', '.', '@', '@', '@', '@', '.'],
            parse("..@@.@@@@.")
        );
    }

    #[test]
    fn test_find_adjacent_cells() {
        assert_eq!(
            vec![
                (1, 2),
                (3, 2),
                (2, 1),
                (2, 3),
                (1, 1),
                (3, 1),
                (1, 3),
                (3, 3)
            ],
            find_adjacent_cells(2, 2, &5, &5)
        );
    }
}
