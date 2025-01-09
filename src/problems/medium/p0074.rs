// Problem: Search a 2D Matrix
// Tags: Array,  Binary Search, Matrix

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for row in matrix.iter() {
        let (first, last) = (row.first().unwrap(), row.last().unwrap());
        if target >= *first && target <= *last {
            let mut left = 0;
            let mut right = row.len() - 1;

            while left <= right {
                let mid = (left + right) / 2;
                if row[mid] == target {
                    return true;
                } else if left == right {
                    return false;
                } else if row[mid] < target {
                    left = mid + 1;
                } else if row[mid] > target {
                    right = mid
                }
            }
        }
    }

    false
}
