// Problem: Min Cost Climbing Stairs
// Tags: Array, Dynamic Programming
pub struct Solution {}
// bottom-up (memory optimized)
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut v = (cost[cost.len() - 2], cost[cost.len() - 1]);
        for i in (0..cost.len() - 2).rev() {
            v = (v.0.min(v.1) + cost[i], v.0);
        }
        return v.0.min(v.1);
    }
}

// bottom-up
// impl Solution {
//     pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
//         let mut cache = vec![0; cost.len() + 1];

//         for i in 2..cache.len() {
//             cache[i] = i32::min(cache[i - 1] + cost[i - 1], cache[i - 2] + cost[i - 2])
//         }

//         cache[cost.len()]
//     }
// }

// top-down
// impl Solution {
//     pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
//         let mut cache = vec![0; cost.len() + 1];
//         i32::min(
//             Solution::dp(&cost, 0, &mut cache),
//             Solution::dp(&cost, 1, &mut cache),
//         )
//     }

//     fn dp(cost: &Vec<i32>, index: usize, cache: &mut Vec<i32>) -> i32 {
//         if index >= cost.len() {
//             return 0;
//         }

//         if cache[index] != 0 {
//             return cache[index];
//         }

//         cache[index] = cost[index]
//             + i32::min(
//                 Solution::dp(cost, index + 1, cache),
//                 Solution::dp(cost, index + 2, cache),
//             );

//         cache[index]
//     }
// }

// brute force
// impl Solution {
//     pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
//         i32::min(Solution::dp(&cost, 0), Solution::dp(&cost, 1))
//     }

//     fn dp(cost: &Vec<i32>, index: usize) -> i32 {
//         if index >= cost.len() {
//             return 0;
//         }
//         cost[index] + i32::min(Solution::dp(cost, index + 1), Solution::dp(cost, index + 2))
//     }
// }
