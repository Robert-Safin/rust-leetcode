// Problem: Neighboring Bitwise XOR
// Tags: Bit Manipulation, Array
pub struct Solution {}

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        // Try both assumptions for original[0]: 0 and 1
        Solution::is_valid(&derived, 0) || Solution::is_valid(&derived, 1)
    }
    fn is_valid(derived: &Vec<i32>, start: i32) -> bool {
        let n = derived.len();
        let mut original = vec![0; n];
        original[0] = start;

        // Reconstruct the array
        for i in 0..n - 1 {
            original[i + 1] = derived[i] ^ original[i];
        }

        // Check cyclic consistency
        derived[n - 1] == (original[n - 1] ^ original[0])
    }
}
