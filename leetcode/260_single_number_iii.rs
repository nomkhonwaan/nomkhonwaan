use std::collections::HashMap;

/// Given an integer array `nums`, in which exactly two elements appear only once and all the other elements appear exactly twice.
/// Find the two elements that appear only once. You can return the answer in any order.
///
/// You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.
///
/// Example 1:
///
/// ```
/// Input: nums = [1,2,1,3,2,5]
/// Output: [3,5]
/// Explanation:  [5, 3] is also a valid answer.
/// ```
///
/// Example 2:
///
/// ```
/// Input: nums = [-1,0]
/// Output: [-1,0]
/// ```
///
/// Example 3:
/// ```
/// Input: nums = [0,1]
/// Output: [1,0]
/// ```
struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut result = HashMap::<i32, u32>::new();

        for n in nums.iter() {
            let m = result.entry(*n).or_insert(0);
            *m += 1;
        }

        result
            .iter()
            .filter(|&(_, v)| *v == 1)
            .map(|(&k, _)| k)
            .collect::<Vec<i32>>()
    }
}
