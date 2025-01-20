// Problem: Surrounded Regions
// Tags: Array, Depth-First Search, Breadth-First Search, Union Find, Matrix
pub struct Solution {}
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut safe = vec![];
        for row_i in 0..board.len() {
            for col_i in 0..board[0].len() {
                if board[row_i][col_i] == 'O' {
                    if row_i == 0
                        || row_i == board.len() - 1
                        || col_i == 0
                        || col_i == board[0].len() - 1
                    {
                        safe.push((row_i, col_i));
                    }
                }
            }
        }

        for (y, x) in safe {
            Solution::bfs(board, y, x);
        }
        for row_i in 0..board.len() {
            for col_i in 0..board[0].len() {
                if board[row_i][col_i] == 'O' {
                    board[row_i][col_i] = 'X';
                }
                if board[row_i][col_i] == 'S' {
                    board[row_i][col_i] = 'O';
                }
            }
        }
    }

    fn bfs(board: &mut Vec<Vec<char>>, y: usize, x: usize) {
        board[y][x] = 'S';
        for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new_y = y as isize + dy;
            let new_x = x as isize + dx;

            if new_y >= 0
                && new_x >= 0
                && new_y < board.len() as isize
                && new_x < board[0].len() as isize
                && board[new_y as usize][new_x as usize] == 'O'
            {
                Solution::bfs(board, new_y as usize, new_x as usize);
            }
        }
    }
}
