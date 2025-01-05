// Problem: Number of Ways to Split Array
// Tags: Array, Prefix Sum

pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut prefix: Vec<i32> = Vec::new();

    let mut current = 0;

    for n in &nums {
        current += n;
        prefix.push(current);
    }

    let mut postfix: Vec<i32> = Vec::new();

    let mut current = 0;

    for n in nums.iter().rev() {
        current += n;
        postfix.push(current);
    }
    postfix.reverse();

    let mut valid: Vec<usize> = vec![];

    for (i, _) in nums.iter().enumerate() {
        if i == nums.len() - 1 {
            continue;
        } else {
            let left_side = prefix[i];
            let right_side = postfix[i + 1];

            if left_side >= right_side {
                valid.push(i);
            }
        }
    }

    valid.len() as i32
}
