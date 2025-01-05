// Problem: Product of Array Except Self
// Tags: Array, Prefix Sum

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();


    let mut prefix = 1;
    for &num in &nums {
        out.push(prefix);
        prefix *= num;
    }

    println!("pre fix {:?}", out);

    // Compute postfix products and update `out` in place
    let mut postfix = 1;
    for (i, &num) in nums.iter().enumerate().rev() {
        out[i] *= postfix;
        postfix *= num;
    }

    out
}
