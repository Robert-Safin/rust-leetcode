// Problem: Coin Change
// Tags: Breadth-First Search, Dynamic Programming, Array
pub struct Solution {}

// top-down
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut mem = vec![-1; amount as usize + 1];
        let out = Solution::dfs(&coins, &mut amount.clone(), &mut mem);
        if out == i32::MAX {
            return -1;
        } else {
            return out;
        }
    }

    fn dfs(coins: &Vec<i32>, amount: &mut i32, mem: &mut Vec<i32>) -> i32 {
        if *amount == 0 {
            return 0;
        };

        if mem[*amount as usize] != -1 {
            return mem[*amount as usize];
        }

        let mut out = i32::MAX;
        for coin in coins {
            if *amount - coin >= 0 {
                out = out.min(
                    Solution::dfs(coins, &mut (*amount - coin), mem)
                        .checked_add(1)
                        .unwrap_or(i32::MAX),
                );
            }
        }
        mem[*amount as usize] = out;
        return mem[*amount as usize];
    }
}

// brute force
// impl Solution {
//     pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
//         let mut amount = amount;
//         let out = Solution::dfs(&coins, &mut amount);

//         if out == i32::MAX {
//             return -1;
//         } else {
//             return out;
//         }
//     }

//     fn dfs(coins: &Vec<i32>, amount: &mut i32) -> i32 {
//         if *amount == 0 {
//             return 0;
//         };

//         let mut out = i32::MAX;
//         for coin in coins {
//             if *amount - coin >= 0 {
//                 out = out.min(
//                     Solution::dfs(coins, &mut (*amount - coin))
//                         .checked_add(1)
//                         .unwrap_or(i32::MAX),
//                 );
//             }
//         }
//         out
//     }
// }
