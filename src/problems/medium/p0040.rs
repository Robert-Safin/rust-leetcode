// Problem: Combination Sum II
// Tags: Array, Backtracking

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrack(
        candidates: &Vec<i32>,
        vec: &mut Vec<i32>,
        target: i32,
        index: usize,
        results: &mut Vec<Vec<i32>>,
        used: &mut Vec<bool>,
    ) {
        println!("{:?}", used);
        let sum = vec.iter().sum::<i32>();

        if sum == target {
            results.push(vec.to_vec());
            return;
        }
        if sum > target || index >= candidates.len() {
            return;
        }

        for i in index..candidates.len() {
            // Skip duplicates at the same recursion level
            if i > index && candidates[i] == candidates[i - 1] && !used[i - 1] {
                continue;
            }

            // Mark the current candidate as used
            vec.push(candidates[i]);
            used[i] = true;

            // Recurse to the next level
            backtrack(candidates, vec, target, i + 1, results, used);

            // Backtrack
            vec.pop();
            used[i] = false;
        }
    }

    let mut results: Vec<Vec<i32>> = vec![];
    let mut vec: Vec<i32> = vec![];
    let mut used: Vec<bool> = vec![false; candidates.len()];

    let mut candidates = candidates.clone();
    candidates.sort();

    backtrack(&candidates, &mut vec, target, 0, &mut results, &mut used);
    //println!("{:?}", results);
    results
}
