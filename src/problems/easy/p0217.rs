// Problem: Contains Duplicate
// Tags: Array, Hash Table, Sorting
use std::collections::HashSet;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();

    for v in nums.iter() {
        if set.contains(v) {
            return true;
        } else {
            set.insert(*v);
        }
    }

    return false;
}
