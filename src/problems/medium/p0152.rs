// Problem: Maximum Product Subarray
// Tags: Dynamic Programming, Array
pub struct Solution {}
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        let mut global_max = 1;
        let mut global_min = 1;

        for num in nums {
            let options = [num * global_max, num * global_min, num];
            global_max = *options.iter().max().unwrap();
            global_min = *options.iter().min().unwrap();

            result = result.max(global_max);
        }

        result
    }
}

// brute force
// impl Solution {
//     pub fn max_product(nums: Vec<i32>) -> i32 {
//         let mut max = 0;
//         for left in 0..nums.len() {
//             for right in left..nums.len() {
//                 let slice = &nums[left..=right];
//                 let product = slice.iter().product::<i32>();
//                 println!("{:?} --> {:?}", slice, product);
//                 max = max.max(product)
//             }
//         }
//         max
//     }
// }
