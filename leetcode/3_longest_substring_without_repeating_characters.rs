fn main() {
    assert_eq!(3, Solution::length_of_longest_substring(String::from("abcabcbb")));
    assert_eq!(1, Solution::length_of_longest_substring(String::from("bbbbb")));
    assert_eq!(3, Solution::length_of_longest_substring(String::from("pwwkew")));
}

/// Given a string s, find the length of the longest substring without repeating characters.
///
/// Example 1:
///
/// ```
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
/// ```
///
/// Example 2:
///
/// ```
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
/// ```
///
/// Example 3:
///
/// ```
/// Input: s = "pwwkew"
/// Output: 3
/// Explanation: The answer is "wke", with the length of 3.
/// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
/// ```
///
/// Constraints:
/// 0 <= s.length <= 5 * 104
/// s consists of English letters, digits, symbols and spaces.
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest_substring = 0usize;
        let chars: Vec<char> = s.chars().collect();

        for (i, n) in chars.iter().enumerate() {
            let mut substring: Vec<char> = vec![*n];

            for m in chars.iter().skip(i + 1) {
                if substring.contains(&m) {
                    break;
                }
                substring.push(*m);
            }

            let len = substring.len();
            
            // no need to find more,
            // this result length is equal to the given string.
            if len == s.len() {
                return len as i32;
            }

            if len > longest_substring {
                longest_substring = len;
            }
        }

        longest_substring as i32
    }
}