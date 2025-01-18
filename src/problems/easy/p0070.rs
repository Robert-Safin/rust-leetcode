pub struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = vec![-1; (n + 1) as usize];
        Solution::dp(n, &mut memo)
    }

    pub fn dp(step: i32, memo: &mut Vec<i32>) -> i32 {
        if step == 0 {
            return 1;
        }
        if step < 0 {
            return 0;
        }

        if memo[step as usize] != -1 {
            return memo[step as usize];
        }

        memo[step as usize] = Solution::dp(step - 1, memo) + Solution::dp(step - 2, memo);

        memo[step as usize]
    }
}
