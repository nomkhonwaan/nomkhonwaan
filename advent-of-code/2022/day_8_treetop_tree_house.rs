use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut grid: Vec<Vec<u32>> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let mut column: Vec<u32> = vec![];
                for tree in line.chars() {
                    column.push(tree.to_digit(10).unwrap());
                }
                grid.push(column);
            }
        }
    }

    let mut visible_trees: Vec<(usize, usize)> = vec![];
    let m = grid[0].len() - 1;
    let n = grid.len() - 1;

    let mut scenic_scores: Vec<u32> = vec![];

    for i in 1..m {
        for j in 1..n {
            let v = visibility_vectors(&grid, i, j);
            if is_visible(&v[0], &v[1], &v[2], &v[3]) {
                visible_trees.push((i, j));
            }

            scenic_scores.push(
                scenic_score(&v[0])
                    * scenic_score(&v[1])
                    * scenic_score(&v[2])
                    * scenic_score(&v[3])
            );
        }
    }

    println!("first part answer is: {}", visible_trees.len() + edge_trees(&grid));
    println!("second part answer is: {}", scenic_scores.iter().max().unwrap());
}

fn edge_trees(grid: &Vec<Vec<u32>>) -> usize {
    (grid.len() - 2) * 2 // left + right
        + grid[0].len() * 2 // top + bottom
}

fn visibility_vectors(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> Vec<Vec<u32>> {
    let tree = grid[i][j];
    let m = grid.len();
    let mut column: Vec<u32> = vec![];
    let row: Vec<u32>;

    // column
    for k in 0..m {
        column.push(match tree <= grid[k][j] {
            true => 1,
            _ => 0,
        });
    }

    // row
    row = grid[i]
        .iter()
        .map(|v| match tree <= *v {
            true => 1,
            _ => 0,
        })
        .collect();

    let mut top: Vec<u32> = column.clone().into_iter().take(i).collect();
    let bottom: Vec<u32> = column.clone().into_iter().skip(i + 1).collect();
    let mut left: Vec<u32> = row.clone().into_iter().take(j).collect();
    let right: Vec<u32> = row.clone().into_iter().skip(j + 1).collect();

    top.reverse();
    left.reverse();

    vec![top, bottom, left, right]
}

fn is_visible(top: &Vec<u32>, bottom: &Vec<u32>, left: &Vec<u32>, right: &Vec<u32>) -> bool {
    top.iter().sum::<u32>() == 0
        || bottom.iter().sum::<u32>() == 0
        || left.iter().sum::<u32>() == 0
        || right.iter().sum::<u32>() == 0
}

fn scenic_score(v: &Vec<u32>) -> u32 {
    let mut score = 0u32;
    for v in v.iter() {
        score += 1;
        if *v == 1 {
            return score;
        }
    }
    score
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_vectors() {
        let grid = &vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(vec![vec![0, 0, 1], vec![1], vec![1, 0, 0], vec![1]], visibility_vectors(&grid, 3, 3));
    }

    #[test]
    fn test_edge_trees() {
        let grid = &vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(16, edge_trees(&grid));
    }

    #[test]
    fn test_is_visible() {
        assert_eq!(false, is_visible(
            &vec![0, 0, 1],
            &vec![1],
            &vec![1, 0, 0],
            &vec![1],
        ));
    }

    #[test]
    fn test_scenic_score() {
        assert_eq!(3, scenic_score(&vec![0, 0, 1]));
        assert_eq!(3, scenic_score(&vec![0, 0, 1, 1]));
        assert_eq!(1, scenic_score(&vec![1]));
    }
}
