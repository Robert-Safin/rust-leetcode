pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0_usize;
    let mut right = nums.len();
    let mut idx: usize;

    while left < right {
        idx = (left + right) / 2;
        if nums[idx] == target {
            return idx as i32;
        } else if nums[idx] < target {
            left = idx + 1;
        } else {
            right = idx;
        }
    }
    -1
}

// pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//     let out = binary_search(&nums, target, 0, nums.len() - 1);
//     println!("{:?}", out);
//     out
// }
// fn binary_search(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> i32 {
//     if left > right {
//         return -1;
//     }

//     let mid = left + (right - left) / 2;

//     if nums[mid] == target {
//         return mid as i32;
//     } else if nums[mid] < target {
//         return binary_search(nums, target, mid + 1, right);
//     } else if mid > 0 {
//         return binary_search(nums, target, left, mid - 1);
//     } else {
//         return -1; // Prevent underflow when mid == 0
//     }
// }
