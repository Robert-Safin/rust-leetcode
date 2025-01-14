// Problem: Word Search
// Tags: Array, String, Backtracking, Depth-First Search, Matrix
pub struct Solution;
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        let word_chars: Vec<char> = word.chars().collect();
        let mut visited = vec![vec![false; cols]; rows];

        for row in 0..rows {
            for col in 0..cols {
                if Solution::dfs(&board, &word_chars, row, col, 0, &mut visited) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        word: &[char],
        y: usize,
        x: usize,
        index: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        // Check if we've matched the whole word
        if index == word.len() {
            return true;
        }

        // Boundary and condition checks
        if y >= board.len() || x >= board[0].len() || visited[y][x] || board[y][x] != word[index] {
            return false;
        }

        // Mark this cell as visited
        visited[y][x] = true;

        // Explore adjacent cells (up, down, left, right)
        for (adj_y, adj_x) in [
            (y + 1, x),
            (y, x + 1),
            (y.wrapping_sub(1), x),
            (y, x.wrapping_sub(1)),
        ] {
            if Solution::dfs(board, word, adj_y, adj_x, index + 1, visited) {
                return true;
            }
        }

        // Backtrack: unmark this cell
        visited[y][x] = false;

        false
    }
}
