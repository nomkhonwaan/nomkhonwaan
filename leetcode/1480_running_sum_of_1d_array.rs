fn main() {
    assert_eq!(vec![1, 3, 6, 10], Solution::running_sum(vec![1, 2, 3, 4]));
    assert_eq!(vec![1, 2, 3, 4, 5], Solution::running_sum(vec![1, 1, 1, 1, 1]));
    assert_eq!(vec![3, 4, 6, 16, 17], Solution::running_sum(vec![3, 1, 2, 10, 1]));
}

/// Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).
///
/// Return the running sum of nums.
///
/// Example 1:
///
/// ```
/// Input: nums = [1,2,3,4]
/// Output: [1,3,6,10]
/// Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
/// ```
///
/// Example 2:
///
/// ```
/// Input: nums = [1,1,1,1,1]
/// Output: [1,2,3,4,5]
/// Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
/// ```
///
/// Example 3:
///
/// ```
/// Input: nums = [3,1,2,10,1]
/// Output: [3,4,6,16,17]
/// ```
///
/// Constraints:
///
/// 1 <= nums.length <= 1000
/// -10^6 <= nums[i] <= 10^6
struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; nums.len()];
        for (i, j) in nums.into_iter().enumerate() {
            if i == 0 {
                result[i] = j;
            } else {
                result[i] = result[i - 1] + j;
            }
        }
        result
    }
}
