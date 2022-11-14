fn main() {
    assert_eq!(vec![vec![-1, -1, 2], vec![-1, 0, 1]], Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    assert_eq!(Vec::<Vec<i32>>::new(), Solution::three_sum(vec![0, 1, 1]));
    assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(vec![0, 0, 0]));
}

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // redeclare for mutation
        let mut nums = nums;
        nums.sort();

        let max = nums.len();
        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..max - 2 {
            // skip duplicates from left
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = max - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];

                if sum == 0 {
                    result.push(vec![nums[i], nums[j], nums[k]]);
                    k -= 1;

                    // skip duplicates from right
                    if j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else if sum > 0 {
                    k -= 1;
                } else {
                    j += 1;
                }
            }
        }

        result
    }
}
