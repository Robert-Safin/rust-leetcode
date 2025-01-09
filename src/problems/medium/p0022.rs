// Problem: Generate Parentheses
// Tags: String, Dynamic Programming, Backtracking

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut strings: Vec<String> = vec![];
    let mut string = String::new();
    recurse(&mut string, &mut strings, n, 0, 0);
    strings
}
pub fn recurse(
    string: &mut String,
    strings: &mut Vec<String>,
    n: i32,
    open_count: i32,
    close_count: i32,
) {
    if string.len() == (2 * n) as usize {
        strings.push(string.clone());
        return;
    }
    if open_count < n {
        string.push('(');
        recurse(string, strings, n, open_count + 1, close_count);
        string.pop();
    }

    if close_count < open_count {
        string.push(')');
        recurse(string, strings, n, open_count, close_count + 1);
        string.pop();
    }
}
