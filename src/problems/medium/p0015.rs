// Problem: 3Sum
// Tags: Array, Two Pointers, Sorting

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    nums.sort();

    let mut out: Vec<Vec<i32>> = Vec::new();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                out.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;

                // Skip duplicates for the two-pointer logic
                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    out
}
