#[allow(unused_imports)]
use rust_leetcode::add_problem::add_problem;
#[allow(unused_imports)]
use rust_leetcode::problems::{easy, hard, medium};

fn main() {
    // add_problem(
    //     2559,
    //     "Count Vowel Strings in Ranges",
    //     &["Array", "Prefix Sum", "String"],
    //     "medium",
    // );

    let out = medium::p2559::vowel_strings(
        vec![
            "aba".to_string(),
            "bcb".to_string(),
            "ece".to_string(),
            "aa".to_string(),
            "e".to_string(),
        ],
        vec![vec![0, 2], vec![1, 4], vec![1, 1]],
    );
    println!("{:?}", out);
}
