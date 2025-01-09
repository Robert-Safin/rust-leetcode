#[allow(unused_imports)]
use rust_leetcode::add_problem::add_problem;
#[allow(unused_imports)]
use rust_leetcode::problems::{easy, hard, medium};

fn main() {
    // add_problem(
    //     74,
    //     "Search a 2D Matrix",
    //     &["Array", " Binary Search", "Matrix"],
    //     "medium",
    // );

    let out = medium::p0074::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        13,
    );

    //let out = medium::p0074::search_matrix(vec![vec![1]], 1);

    println!("{:?}", out);
}
