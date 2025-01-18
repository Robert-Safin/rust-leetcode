// Problem: Decode Ways
// Tags: String, Dynamic Programming
pub struct Solution {}
// top-down
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut memo = vec![-1; s.len()];
        Solution::dp(&s.chars().collect(), 0, &mut memo)
    }

    fn dp(s: &Vec<char>, index: usize, memo: &mut Vec<i32>) -> i32 {
        if index >= s.len() {
            return 1;
        }
        if s[index] == '0' {
            return 0;
        }
        if memo[index] != -1 {
            return memo[index];
        }

        let mut res = Solution::dp(s, index + 1, memo);

        if index + 1 < s.len()
            && (s[index] == '1' || (s[index] == '2' && s[index + 1].to_digit(10).unwrap() < 7))
        {
            res += Solution::dp(s, index + 2, memo);
        }

        memo[index] = res;
        res
    }
}

// impl Solution {
//     pub fn num_decodings(s: String) -> i32 {
//         Solution::dp(&s.chars().collect(), 0)
//     }

//     fn dp(s: &Vec<char>, index: usize) -> i32 {
//         if index >= s.len() {
//             return 1;
//         }
//         if s[index] == '0' {
//             return 0;
//         }

//         let mut res = Solution::dp(s, index + 1);

//         if index + 1 < s.len()
//             && (s[index] == '1' || (s[index] == '2' && s[index + 1].to_digit(10).unwrap() < 7))
//         {
//             res += Solution::dp(s, index + 2);
//         }

//         res
//     }
// }
