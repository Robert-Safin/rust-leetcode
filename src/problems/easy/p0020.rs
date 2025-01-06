// Problem: Valid Parentheses
// Tags: Stack, Sorting

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for char in s.chars() {
        if ['{', '[', '('].contains(&char) {
            stack.push(char);
        } else {
            if let Some(last) = stack.last() {
                match last {
                    '{' => {
                        if char == '}' {
                            stack.pop();
                        } else {
                            return false;
                        }
                    }
                    '[' => {
                        if char == ']' {
                            stack.pop();
                        } else {
                            return false;
                        }
                    }
                    '(' => {
                        if char == ')' {
                            stack.pop();
                        } else {
                            return false;
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                return false;
            }
        }
    }
    if stack.len() == 0 {
        true
    } else {
        false
    }
}
