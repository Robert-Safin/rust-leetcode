// Problem: Top K Frequent Elements
// Tags: Array, Hash Table, Divide and Conquer, Sorting, Heap (Priority Queue), Bucket Sort, Counting, Quickselect
use std::collections::HashMap;
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for n in nums {
        if let Some(v) = map.get_mut(&n) {
            *v += 1;
        } else {
            map.insert(n, 1);
        }
    }

    let mut sorted: Vec<(i32, i32)> = map
        .iter()
        .map(|v| (v.1.to_owned(), v.0.to_owned()))
        .collect();

    sorted.sort();

    let mut out: Vec<i32> = vec![];

    for _ in 0..k {
        out.push(sorted.pop().unwrap().1);
    }

    out
}
