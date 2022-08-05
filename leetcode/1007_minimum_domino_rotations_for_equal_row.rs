use std::cmp::min;

fn main() {
    assert_eq!(2, Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]));
    assert_eq!(-1, Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]));
    assert_eq!(1, Solution::min_domino_rotations(vec![1, 2, 1, 1, 1, 2, 2, 2], vec![2, 1, 2, 2, 2, 2, 2, 2]));
    assert_eq!(3, Solution::min_domino_rotations(vec![2, 2, 2, 4, 4, 4], vec![4, 4, 4, 2, 3, 2]));
}

/// In a row of dominoes, tops[i] and bottoms[i] represent the top and bottom halves of the ith domino.
/// (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)
/// We may rotate the ith domino, so that tops[i] and bottoms[i] swap values.
/// Return the minimum number of rotations so that all the values in tops are the same, or all the values in bottoms are the same.
/// If it cannot be done, return -1.
///
/// Example 1:
///
/// ```
/// Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
/// Output: 2
/// Explanation:
/// The first figure represents the dominoes as given by tops and bottoms: before we do any rotations.
/// If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2, as indicated by the second figure.
/// ```
///
///
/// Example 2:
///
/// ```
/// Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
/// Output: -1
/// Explanation:
/// In this case, it is not possible to rotate the dominoes to make one row of values equal.
/// ```
///
/// Constraints:
///
/// 2 <= tops.length <= 2 * 104
/// bottoms.length == tops.length
/// 1 <= tops[i], bottoms[i] <= 6
struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut dominoes: Vec<Domino> = vec![];
        for (i, top) in tops.iter().enumerate() {
            dominoes.push(Domino::new(*top, bottoms[i]));
        }

        let target_values = find_target_values(&dominoes);

        // return -1 since there is no target value.
        if target_values.len() == 0 {
            return -1;
        }

        let mut minimum_rotation = -1i32;
        for target_value in target_values {
            let rotation = rotate(target_value, &dominoes);

            // both top and bottom rotation still unable to rotate to this target value,
            // try another target value instead.
            if rotation == -1 {
                continue;
            }

            if minimum_rotation == -1 || rotation < minimum_rotation {
                minimum_rotation = rotation;
            }
        }

        minimum_rotation
    }
}

#[derive(Default, Debug)]
struct Domino {
    top: i32,
    bottom: i32,
}

impl Domino {
    fn new(top: i32, bottom: i32) -> Self {
        Domino { top, bottom, ..Domino::default() }
    }

    fn is_double(&self) -> bool {
        self.top == self.bottom
    }
}

type Dominoes = Vec<Domino>;

// Find target values to rotate to.
fn find_target_values(dominoes: &Dominoes) -> Vec<i32> {
    let mut target_values: Vec<i32> = vec![];

    for domino in dominoes {
        if domino.is_double() {
            return vec![domino.top];
        }
    }

    target_values.append(&mut find_target_values_from_pair(&dominoes[0], &dominoes[1]));
    target_values
}

fn find_target_values_from_pair(first_domino: &Domino, second_domino: &Domino) -> Vec<i32> {
    let mut target_values: Vec<i32> = vec![];

    if first_domino.top == second_domino.top || first_domino.top == second_domino.bottom {
        target_values.push(first_domino.top);
    }
    if first_domino.bottom == second_domino.top || first_domino.bottom == second_domino.bottom {
        target_values.push(first_domino.bottom);
    }

    target_values
}

fn rotate(target_value: i32, dominoes: &Dominoes) -> i32 {
    let mut m = 0i32;
    let mut n = 0i32;
    for domino in dominoes {
        // either top or bottom are not rotate to the target value.
        if domino.top != target_value && domino.bottom != target_value {
            return -1;
        }
        if domino.top != target_value {
            m += 1;
        }
        if domino.bottom != target_value {
            n += 1;
        }
    }
    min(m, n)
}