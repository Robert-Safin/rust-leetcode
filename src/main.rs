#[allow(unused_imports)]
use rust_leetcode::add_problem::add_problem;
#[allow(unused_imports)]
use rust_leetcode::problems::{easy, hard, medium};

fn main() {
    // add_problem(
    //     2657,
    //     "Find the Prefix Common Array of Two Arrays",
    //     &["Hash Table", "Array", "Bit Manipulation"],
    //     "medium",
    // );

    let out =
        medium::p2657::Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]);
    println!("{:?}", out);
}
