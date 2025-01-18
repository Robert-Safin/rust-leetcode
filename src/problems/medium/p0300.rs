// Problem: Longest Increasing Subsequence
// Tags: Dynamic Programming, Array, Binary Search
pub struct Solution {}
// binary search
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sub = Vec::new();

        for num in nums {
            // Use binary search to find the position of `num` in `sub`
            let pos = sub.binary_search(&num).unwrap_or_else(|x| x);

            if pos < sub.len() {
                sub[pos] = num; // Replace the element at `pos`
            } else {
                sub.push(num); // Append `num` to extend the LIS
            }
            println!("{:?}", sub);
        }

        sub.len() as i32 // The length of `sub` is the LIS length
    }
}
// dp
// impl Solution {
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         let mut memo = vec![1; nums.len()];

//         for outer in (0..nums.len()).rev() {
//             for inner in outer + 1..nums.len() {
//                 if nums[outer] < nums[inner] {
//                     memo[outer] = i32::max(memo[outer], 1 + memo[inner])
//                 }
//             }
//         }
//         return memo.into_iter().max().unwrap_or(1);
//     }
// }
// brute-force
// impl Solution {
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         let mut max_len = 0;

//         for i in 0..nums.len() {
//             max_len = max_len.max(Solution::dp(&nums, i));
//         }

//         max_len
//     }

//     fn dp(nums: &Vec<i32>, index: usize) -> i32 {
//         let mut max_len = 1;

//         for next in index + 1..nums.len() {
//             if nums[next] > nums[index] {
//                 max_len = max_len.max(1 + Solution::dp(nums, next));
//             }
//         }

//         max_len
//     }
// }
