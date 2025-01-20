// Problem: First Completely Painted Row or Column
// Tags: Array, Hash Table, Matrix
pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        // 1. Parse matrix and store where each value is located in the matrix for quick access
        let mut hash: HashMap<i32, (usize, usize)> = HashMap::new();
        for row_i in 0..mat.len() {
            for col_i in 0..mat[0].len() {
                hash.insert(mat[row_i][col_i], (row_i, col_i));
            }
        }

        // 2. Frequency counters for rows and columns, which start at 0 and increment by 1 when corresponding row/col is painited.
        // This means there is no need to mutate the matrix and check if entire row/col is painted at each loop of arr.
        let mut rows_freq = vec![0; mat.len()];
        let mut cols_freq = vec![0; mat[0].len()];

        // 3. Parse arr and update frequency counters
        for (i, n) in arr.iter().enumerate() {
            // Get the row index and col index of the cell that is getting painted
            // Each painted cell will increment a value in both rows_freq & col_freq since it has both row index and col index.
            let (y, x) = hash.get(&n).unwrap();

            // 4. update the correct row frequency
            rows_freq[*y] += 1;
            // 5. Check the frequency of the row that was just updated, if its equal to mat[0].len(), the entire row is painted, return arr[i] as i32
            if rows_freq[*y] == mat[0].len() {
                return i as i32;
            }
            // 6. Repeat 4 + 5 for columns
            cols_freq[*x] += 1;
            if cols_freq[*x] == mat.len() {
                return i as i32;
            }
        }
        // Assuming a valid answer does exist in the input, this should not be reached.
        unreachable!()
    }
}
