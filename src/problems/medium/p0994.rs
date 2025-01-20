// Problem: Rotting Oranges
// Tags: Array, Breadth-First Search, Matrix
//
use std::collections::VecDeque;
pub struct Solution {}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh = 0;
        let mut time = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                match grid[row][col] {
                    2 => queue.push_back((row, col)),
                    1 => fresh += 1,
                    _ => (),
                }
            }
        }

        if fresh == 0 {
            return 0;
        }

        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        // create snapshot of queue's length, pop-left that many, push-right new points
        while !queue.is_empty() {
            let mut rotten_inside_queue = queue.len();

            while rotten_inside_queue > 0 {
                let (y, x) = queue.pop_front().unwrap();

                for (dy, dx) in directions {
                    let new_y = y as isize + dy;
                    let new_x = x as isize + dx;

                    if new_y >= 0
                        && new_x >= 0
                        && new_y < grid.len() as isize
                        && new_x < grid[0].len() as isize
                        && grid[new_y as usize][new_x as usize] == 1
                    {
                        grid[new_y as usize][new_x as usize] = 2;
                        fresh -= 1;
                        queue.push_back((new_y as usize, new_x as usize));
                    }
                }
                rotten_inside_queue -= 1;
            }

            time += 1;
        }

        if fresh > 0 {
            -1
        } else {
            time - 1
        }
    }
}
