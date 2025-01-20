// Problem: Trapping Rain Water
// Tags: Array, Two Pointers, Dynamic Programming, Stack, Monotonic Stack
pub struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (height[left], height[right]);

        let mut res = 0;

        while left < right {
            if left_max < right_max {
                left += 1;
                left_max = i32::max(left_max, height[left]);
                res += left_max - height[left];
            } else {
                right -= 1;
                right_max = i32::max(right_max, height[right]);
                res += right_max - height[right];
            }
        }
        res
    }
}
