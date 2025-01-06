#[allow(unused_imports)]
use rust_leetcode::add_problem::add_problem;
#[allow(unused_imports)]
use rust_leetcode::problems::{easy, hard, medium};

fn main() {
    // add_problem(
    //     150,
    //     "Evaluate Reverse Polish Notation",
    //     &["Stack", "Design", "Math"],
    //     "medium",
    // );
    let out = medium::p0150::eval_rpn(vec![
        "2".to_owned(),
        "1".to_owned(),
        "+".to_owned(),
        "3".to_owned(),
        "*".to_owned(),
    ]);

    println!("{:?}", out);
}
