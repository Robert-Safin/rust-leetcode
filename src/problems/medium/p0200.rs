// Problem: Number of Islands
// Tags: Array, Breadth-First Search, Depth-First Search, Matrix, Union find
pub struct Solution {}
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;

        for col_i in 0..grid.len() {
            for row_i in 0..grid[0].len() {
                if grid[col_i][row_i] == '1' {
                    islands += 1;
                    Solution::dfs(col_i, row_i, &mut grid);
                }
            }
        }

        islands
    }

    fn dfs(y: usize, x: usize, grid: &mut Vec<Vec<char>>) {
        grid[y][x] = '0';

        for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new_y = y as isize + dy;
            let new_x = x as isize + dx;

            if new_y >= 0
                && new_x >= 0
                && new_y < grid.len() as isize
                && new_x < grid[0].len() as isize
                && grid[new_y as usize][new_x as usize] == '1'
            {
                Solution::dfs(new_y as usize, new_x as usize, grid);
            }
        }
    }
}
