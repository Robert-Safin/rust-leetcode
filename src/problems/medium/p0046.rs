// Problem: Permutations
// Tags: Array, Backtracking

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];

    backtrack(&nums, &mut current, &mut used, &mut result);

    result
}
fn backtrack(
    nums: &Vec<i32>,
    current: &mut Vec<i32>,
    used: &mut Vec<bool>,
    result: &mut Vec<Vec<i32>>,
) {
    if used.iter().all(|&x| x == true) {
        result.push(current.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }

        used[i] = true;
        current.push(nums[i]);
        backtrack(nums, current, used, result);
        current.pop();
        used[i] = false;
    }
}
