// Problem: Max Area of Island
// Tags: Array, Breadth-First Search, Depth-First Search, Matrix, Union find
pub struct Solution {}
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut max_size = 0;
        for row_i in 0..grid.len() {
            for col_i in 0..grid[0].len() {
                if grid[row_i][col_i] == 1 {
                    max_size = max_size.max(Solution::dfs(&mut grid, row_i, col_i));
                }
            }
        }
        max_size
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, y: usize, x: usize) -> i32 {
        if grid[y][x] == 0 {
            return 0;
        }

        grid[y][x] = 0;

        let mut sum = 1;
        for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new_y = y as isize + dy;
            let new_x = x as isize + dx;

            if new_y >= 0
                && new_x >= 0
                && new_y < grid.len() as isize
                && new_x < grid[0].len() as isize
                && grid[new_y as usize][new_x as usize] == 1
            {
                sum += Solution::dfs(grid, new_y as usize, new_x as usize);
            }
        }
        return sum;
    }
}
