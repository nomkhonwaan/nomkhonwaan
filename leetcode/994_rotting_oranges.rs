use std::collections::VecDeque;

fn main() {
    assert_eq!(4, Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]));
    assert_eq!(-1, Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]));
    assert_eq!(0, Solution::oranges_rotting(vec![vec![0, 2]]));
}

/// You are given an m x n grid where each cell can have one of three values:
///
/// 0 representing an empty cell,
/// 1 representing a fresh orange, or
/// 2 representing a rotten orange.
/// Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
///
/// Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
///
/// Example 1:
///
/// ```
/// Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
/// Output: 4
/// ```
///
/// Example 2:
///
/// ```
/// Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
/// Output: -1
/// Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
/// ```
///
/// Example 3:
///
/// ```
/// Input: grid = [[0,2]]
/// Output: 0
/// Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
/// ```
///
/// Constraints:
///
/// m == grid.length
/// n == grid[i].length
/// 1 <= m, n <= 10
/// grid[i][j] is 0, 1, or 2.
struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        // shadow redeclaration for mutating grid variable
        let mut grid = grid;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh = 0u32;
        let mut times = 0i32;
        let max_m = grid.len();
        let max_n = grid[0].len();

        for i in 0..max_m {
            for j in 0..max_n {
                match grid[i][j] {
                    1 => {
                        fresh += 1;
                    }
                    2 => {
                        queue.push_back((i, j));
                    }
                    _ => {}
                }
            }
        }

        // there are already no fresh oranges at minute 0,
        // the answer is just 0
        if fresh == 0 {
            return 0;
        }

        while !queue.is_empty() {
            times += 1;
            let size = queue.len();
            // this loop will try to spread rotting oranges into others at the same time
            for _ in 0..size {
                let (i, j) = queue.pop_front().unwrap();
                let vertices = find_all_adjacent_vertices(i, j, max_m, max_n);
                for v in vertices {
                    // if orange is not rotted, mark it as rotten and enqueue it
                    if grid[v.0][v.1] == 1 {
                        grid[v.0][v.1] = 2;
                        queue.push_back((v.0, v.1));
                        fresh -= 1;
                    }
                }
            }

            if fresh == 0 {
                return times;
            }
        }

        if fresh > 0 {
            return -1;
        }

        times - 1
    }
}

fn find_all_adjacent_vertices(
    i: usize,
    j: usize,
    max_m: usize,
    max_n: usize,
) -> Vec<(usize, usize)> {
    let mut vertices = vec![];
    // right
    if j < max_n - 1 {
        vertices.push((i, j + 1));
    }
    // bottom
    if i < max_m - 1 {
        vertices.push((i + 1, j));
    }
    // left
    if j > 0 {
        vertices.push((i, j - 1));
    }
    // top
    if i > 0 {
        vertices.push((i - 1, j));
    }
    vertices
}
