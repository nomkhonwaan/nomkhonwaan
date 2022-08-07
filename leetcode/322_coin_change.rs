use std::cmp::min;

fn main() {
    assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
    assert_eq!(-1, Solution::coin_change(vec![2], 3));
    assert_eq!(0, Solution::coin_change(vec![1], 0));
    assert_eq!(4, Solution::coin_change(vec![20, 12, 1], 48));
    assert_eq!(2, Solution::coin_change(vec![2, 3, 6, 7], 12));
    assert_eq!(2, Solution::coin_change(vec![9, 6, 5, 1], 11));
    assert_eq!(20, Solution::coin_change(vec![186, 419, 83, 408], 6249));
}

/// You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
/// Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
/// You may assume that you have an infinite number of each kind of coin.
///
/// Example 1:
///
/// ```
/// Input: coins = [1,2,5], amount = 11
/// Output: 3
/// Explanation: 11 = 5 + 5 + 1
/// ```
///
/// Example 2:
///
/// ```
/// Input: coins = [2], amount = 3
/// Output: -1
/// ```
///
/// Example 3:
///
/// ```
/// Input: coins = [1], amount = 0
/// Output: 0
/// ```
///
/// Constraints:
///
/// 1 <= coins.length <= 12
/// 1 <= coins[i] <= 231 - 1
/// 0 <= amount <= 104
struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
		// use amount plus 1 as max value to avoiding overflow
		let max = amount + 1;
        let mut table = vec![max; amount as usize + 1];
		
		// base value 
		table[0] = 0;

        // inclusive loop until an amount
        for i in 1..amount + 1 {
            for coin in coins.iter() {
                if *coin <= i {
                    table[i as usize] = min(table[i as usize], table[(i - coin) as usize] + 1);
                }
            }
        }

        let res = table[amount as usize];
        if res > amount {
            return -1;
        }

        res
    }
}
