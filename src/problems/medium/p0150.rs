// Problem: Evaluate Reverse Polish Notation
// Tags: Stack, Design, Math

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];

    for token in tokens {
        if let Ok(number) = token.parse::<i32>() {
            stack.push(number);
        } else {
            let (right, left) = (stack.pop().unwrap(), stack.pop().unwrap());
            let result = match token.as_str() {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                "/" => left / right,
                _ => unreachable!(),
            };
            stack.push(result);
        }
    }
    stack.pop().unwrap()
}
