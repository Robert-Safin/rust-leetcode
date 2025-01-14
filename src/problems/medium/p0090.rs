// Problem: Subsets II
// Tags: Array, Backtracking, Bit Manipulation

pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut ans: Vec<Vec<i32>> = vec![];
    solve(&nums, 0, &mut vec![], &mut ans);
    ans
}
fn solve(nums: &Vec<i32>, ind: usize, subset: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    ans.push(subset.to_vec());
    for i in ind..nums.len() {
        if i != ind && nums[i] == nums[i - 1] {
            continue;
        }
        subset.push(nums[i]);
        solve(nums, i + 1, subset, ans);
        subset.pop();
    }
}
