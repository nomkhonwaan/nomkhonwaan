use advent_of_code_2025::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_lines(&args[1]).unwrap();
    let grid = parse_grid(&input);

    println!("First part answer: {}", cal_first_part_answer(&grid));
}

fn parse_grid(input: &[String]) -> Vec<Vec<u8>> {
    input.iter().map(|line| line.bytes().collect()).collect()
}

fn cal_first_part_answer(grid: &[Vec<u8>]) -> usize {
    let start = find_start(grid).expect("grid must have a start position");
    let mut split_count = 0;
    let mut beams = vec![start];
    let mut visited_splitters = vec![vec![false; grid[0].len()]; grid.len()];

    while let Some((row, col)) = beams.pop() {
        let mut r = row + 1;

        while r < grid.len() {
            match grid[r][col] {
                b'^' => {
                    if !visited_splitters[r][col] {
                        visited_splitters[r][col] = true;
                        split_count += 1;
                        if col > 0 {
                            beams.push((r, col - 1));
                        }
                        if col + 1 < grid[0].len() {
                            beams.push((r, col + 1));
                        }
                    }
                    break;
                }
                _ => {
                    r += 1;
                }
            }
        }
    }

    split_count
}

fn find_start(grid: &[Vec<u8>]) -> Option<(usize, usize)> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == b'S' {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0].len(), 15);
        assert_eq!(grid[0][7], b'S');
        assert_eq!(grid[2][7], b'^');
        assert_eq!(grid[4][6], b'^');
        assert_eq!(grid[4][8], b'^');
    }

    #[test]
    fn test_find_start() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
        ];
        let grid = parse_grid(&input);
        let (row, col) = find_start(&grid).unwrap();
        assert_eq!((row, col), (0, 7));
    }

    #[test]
    fn test_find_start_no_s() {
        let input = vec!["...............".to_string(), "...............".to_string()];
        let grid = parse_grid(&input);
        assert!(find_start(&grid).is_none());
    }

    #[test]
    fn test_no_splitters() {
        let input = vec!["S..".to_string(), "...".to_string(), "...".to_string()];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 0);
    }

    #[test]
    fn test_no_splitters_single_column() {
        let input = vec![
            "S".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 0);
    }

    #[test]
    fn test_one_splitter() {
        let input = vec!["S".to_string(), "^".to_string(), ".".to_string()];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 1);
    }

    #[test]
    fn test_one_splitter_offset() {
        let input = vec!["S..".to_string(), "...".to_string(), "..^".to_string()];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 0);
    }

    #[test]
    fn test_two_splitters_in_line() {
        let input = vec![
            "S".to_string(),
            "^".to_string(),
            "^".to_string(),
            ".".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 1);
    }

    #[test]
    fn test_three_splitters_in_line() {
        let input = vec![
            "S".to_string(),
            "^".to_string(),
            "^".to_string(),
            "^".to_string(),
            ".".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 1);
    }

    #[test]
    fn test_diamond_pattern() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
            ".^.^.".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 3);
    }

    #[test]
    fn test_full_example() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 21);
    }
}
