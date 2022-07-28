fn main() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
}

/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
/// You can return the answer in any order.
///
/// Example 1:
///
/// ```
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
/// ```
///
/// Example 2:
///
/// ```
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
/// ```
///
/// Example 3:
///
/// ```
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
/// ```
///
/// Constraints:
/// 2 <= nums.length <= 104
/// -109 <= nums[i] <= 109
/// -109 <= target <= 109
/// Only one valid answer exists.
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i: usize = 0;

        // first loop starting from the first index
        while i < nums.len() {
            let mut j = i + 1;

            // second loop starting from the next item from the first loop
            while j < nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }

                j += 1;
            }

            i += 1;
        }
        
        vec![]
    }
}