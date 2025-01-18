// Problem: House Robber
// Tags: Array, Dynamic Programming
pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut prev = 0; // Max amount robbed up to i-2
        let mut curr = 0; // Max amount robbed up to i-1

        for num in nums {
            let next = i32::max(curr, prev + num); // Rob or skip the current house
            prev = curr; // Update prev to the old curr
            curr = next; // Update curr to the new max
        }

        curr
    }
}
// bottom-up
// impl Solution {
//     pub fn rob(nums: Vec<i32>) -> i32 {
//         if nums.len() == 1 {
//             return nums[0];
//         }
//         let mut cache = vec![-1; nums.len()];

//         cache[0] = nums[0];
//         cache[1] = i32::max(nums[1], nums[0]);

//         for i in 2..nums.len() {
//             cache[i] = i32::max(cache[i - 1], nums[i] + cache[i - 2]);
//         }
//         *cache.last().unwrap()
//     }
// }
// top-down
// impl Solution {
//     pub fn rob(nums: Vec<i32>) -> i32 {
//         let mut cache = vec![-1; nums.len() + 1];
//         Solution::dp(&nums, 0, &mut cache)
//     }

//     fn dp(nums: &Vec<i32>, index: usize, cache: &mut Vec<i32>) -> i32 {
//         if index >= nums.len() {
//             return 0;
//         };

//         if cache[index] != -1 {
//             return cache[index];
//         }

//         cache[index] = i32::max(
//             nums[index] + Solution::dp(nums, index + 2, cache),
//             Solution::dp(nums, index + 1, cache),
//         );

//         return cache[index];
//     }
// }
