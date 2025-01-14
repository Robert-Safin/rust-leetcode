// Problem: Find the Prefix Common Array of Two Arrays
// Tags: Hash Table, Array, Bit Manipulation
pub struct Solution {}
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut counts = vec![0; a.len() + 1];
        let mut out: Vec<i32> = vec![];
        for (a, b) in a.into_iter().zip(b) {
            counts[a as usize] += 1;
            counts[b as usize] += 1;
            out.push(counts.iter().filter(|v| **v == 2).count() as i32);
        }
        out
    }
}
