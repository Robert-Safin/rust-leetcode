// Problem: House Robber II
// Tags: Array, Dynamic Programming
pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut cache_a = vec![-1; nums.len()];
        let mut cache_b = vec![-1; nums.len()];
        let a = &nums[1..];
        let b = &nums[..nums.len() - 1];

        i32::max(
            Solution::dp(a, 0, &mut cache_a),
            Solution::dp(b, 0, &mut cache_b),
        )
    }
    fn dp(nums: &[i32], index: usize, cache: &mut Vec<i32>) -> i32 {
        if index >= nums.len() {
            return 0;
        };
        if cache[index] != -1 {
            return cache[index];
        };
        cache[index] = i32::max(
            Solution::dp(nums, index + 1, cache),
            nums[index] + Solution::dp(nums, index + 2, cache),
        );
        return cache[index];
    }
}
