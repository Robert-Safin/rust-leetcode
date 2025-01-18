// Problem: Minimize XOR
// Tags: Array, Bit Manipulation, Brainteaser
pub struct Solution {}

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let xor_nums1: i32 = nums1.iter().fold(0, |acc, &x| acc ^ x);
        let xor_nums2: i32 = nums2.iter().fold(0, |acc, &x| acc ^ x);

        let len1_is_odd = nums1.len() % 2 == 1;
        let len2_is_odd = nums2.len() % 2 == 1;

        let mut result = 0;

        if len1_is_odd {
            result ^= xor_nums2;
        }

        if len2_is_odd {
            result ^= xor_nums1;
        }

        result
    }
}
