// Problem: Minimum Cost to Make at Least One Valid Path in a Grid
// Tags: Array, Breadth-First Search, Graph, Heap (Priority Queue), Matrix, Shortest Path
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        // Rust's std's only offers a max heap. It could be adapted to be a min heap if values are reversed
        let mut heap = BinaryHeap::new();

        // prevents revisiting cells, since this can never lead to optimal results and can leap to infinity loop
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        // if heap's element has multiple values, the first element is what will determine heap's priority (default but configurable)
        // insert the starting branch, where score and y & x positions all equal 0
        heap.push((Reverse(0), 0, 0));

        // The approach is priority-based Breadth-First Search (BFS) aka Dijkstra's Algorithm:
        // 1. Pop a branch from the heap, since its a min heap, the popped branch will have the lowest score available in the heap.
        while let Some((Reverse(cost), y, x)) = heap.pop() {
            // 2. End case, the bottom-right cell is reached, and the current branch's score is returned.
            // The first time the end is encountered, it is the cheapest path (although there may be other unique equally cheap paths).
            // This is due to the min heap prioritizing returning us branches where the accumulated score is lowest.
            if (y, x) == (grid.len() - 1, grid[0].len() - 1) {
                return cost;
            }

            // 3. if the branch encounteres a previously visited cell, terminate the branch.
            if visited[y][x] {
                continue;
            }

            // 4. Otherwise mark cell as visited.
            visited[y][x] = true;

            // 5. Create the 4 potential neighbours.The neighbours are strategically ordered to allign with the directions in the problem statement (right,left,down,up).
            // This then allows us to also grab the index, add 1 to the index, and match on that value to determine if moving to said neighbour is free.
            for (i, &(dy, dx)) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().enumerate() {
                // using isize over usize is nice since isize will cause a panic if it goes nagative.
                // Temprarily converting usize to isize allows us to create a negative index without runtime panics, and filter it out later on.
                let new_y = y as isize + dy;
                let new_x = x as isize + dx;

                // 6. Filter invalid neighbours which are out of bounds.
                if new_y >= 0
                    && new_x >= 0
                    && (new_y as usize) < grid.len()
                    && (new_x as usize) < grid[0].len()
                {
                    // cast isize back to usize, to allign with heap's exepcted type
                    let new_y = new_y as usize;
                    let new_x = new_x as usize;

                    // 7. Determine if moving to said neighbour is free (matching the direction) or costs 1
                    let new_cost = if grid[y][x] == (i + 1) as i32 {
                        cost
                    } else {
                        cost + 1
                    };

                    // 8. Insert new valid branch
                    heap.push((Reverse(new_cost), new_y, new_x));

                    // 9. Note: the visited does not get unmarked as this is BFS, where as DFS would require unmarking.
                    // Visuallising the progress of the barnches in DFS vs BFS is helpfull to unserstand why that is.
                }
            }
        }

        unreachable!()
    }
}
