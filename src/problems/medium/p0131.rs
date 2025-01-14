// Problem: Palindrome Partitioning
// Tags: String, Dynamic Programming, Backtracking

pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = vec![];
        Solution::backtrack(&s, &mut vec![], &mut result, 0);
        result
    }

    fn backtrack(
        s: &String,
        current: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
        index: usize,
    ) {
        if index == s.len() {
            result.push(current.clone());
            return;
        }

        for i in index..s.len() {
            if Solution::is_palindrome(&s[index..=i]) {
                current.push(s[index..=i].to_string());
                Solution::backtrack(s, current, result, i + 1);
                current.pop();
            }
        }
    }

    fn is_palindrome(s: &str) -> bool {
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            if s.as_bytes()[left] != s.as_bytes()[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
