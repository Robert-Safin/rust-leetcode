// Problem: Check if a Parentheses String Can Be Valid
// Tags: String, Stack, Greedy
//

pub struct Solution {}

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let mut input: Vec<(char, bool, usize)> = vec![];
        let s_chars: Vec<char> = s.chars().collect();
        let locked_chars: Vec<bool> = locked.chars().map(|c| c == '1').collect();
        for i in 0..s.len() {
            input.push((s_chars[i], locked_chars[i], i));
        }

        if input.len() % 2 != 0 {
            return false;
        }

        let mut open_stack: Vec<usize> = vec![];
        let mut wild_stack: Vec<usize> = vec![];

        for (parenthesis, is_locked, i) in input {
            if parenthesis == '(' && is_locked == true {
                open_stack.push(i);
            } else if is_locked == false {
                wild_stack.push(i);
            } else if parenthesis == ')' && is_locked == true {
                if let Some(_open) = open_stack.pop() {
                } else {
                    if let Some(_wild) = wild_stack.pop() {
                    } else {
                        return false;
                    }
                }
            } else {
                panic!("unexpected")
            }
        }

        if open_stack.len() == 0 {
            return true;
        }

        if open_stack.len() > wild_stack.len() {
            return false;
        }

        while open_stack.len() > 0 {
            let open = open_stack.pop().unwrap();
            let wild = wild_stack.pop().unwrap();
            if open > wild {
                return false;
            }
        }

        true
    }
}
