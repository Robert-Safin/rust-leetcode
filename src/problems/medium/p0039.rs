// Problem: Combination Sum
// Tags: Array, Backtracking

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut current: Vec<i32> = vec![];
    backtrack(&candidates, target, &mut current, &mut result, 0);

    result
}
fn backtrack(
    candidates: &Vec<i32>,
    target: i32,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
    index: usize,
) {
    let sum = current.iter().sum::<i32>();
    if sum == target {
        result.push(current.clone());
        return;
    }
    if index >= candidates.len() || sum > target {
        return;
    }
    current.push(candidates[index]);
    backtrack(candidates, target, current, result, index);
    current.pop();
    backtrack(candidates, target, current, result, index + 1);
}
