// Problem: Trapping Rain Water II
// Tags: Array, Breadth-First Search, Heap (Priority Queue), Matrix
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let rows = height_map.len();
        let cols = height_map[0].len();

        if rows < 3 || cols < 3 {
            return 0; // No water can be trapped if the grid is too small
        }

        let mut visited = vec![vec![false; cols]; rows];
        let mut heap = BinaryHeap::new();

        // Push all boundary cells into the heap
        for r in 0..rows {
            for c in [0, cols - 1] {
                heap.push(Reverse((height_map[r][c], r, c)));
                visited[r][c] = true;
            }
        }

        for c in 0..cols {
            for r in [0, rows - 1] {
                heap.push(Reverse((height_map[r][c], r, c)));
                visited[r][c] = true;
            }
        }

        let mut water = 0;
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        // Process the heap
        while let Some(Reverse((height, r, c))) = heap.pop() {
            for &(dr, dc) in &directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                // Check bounds and visited status
                if nr >= 0 && nc >= 0 && (nr as usize) < rows && (nc as usize) < cols {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        let neighbor_height = height_map[nr][nc];
                        water += (height - neighbor_height).max(0);
                        // Push the updated height into the heap
                        heap.push(Reverse((height.max(neighbor_height), nr, nc)));
                    }
                }
            }
        }

        water
    }
}
