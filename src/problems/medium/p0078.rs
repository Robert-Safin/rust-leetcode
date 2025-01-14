// Problem: Subsets
// Tags: Array, Backtracking, Bit Manipulation

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sets: Vec<Vec<i32>> = vec![];
    backtrack(&nums, &mut sets, &mut vec![], 0);

    sets
}
fn backtrack(nums: &Vec<i32>, sets: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, index: usize) {
    if index >= nums.len() {
        sets.push(current.clone());
        return;
    } else {
        current.push(nums[index]);
        backtrack(nums, sets, current, index + 1);
        current.pop();
        backtrack(nums, sets, current, index + 1);
    }
}
