use std::collections::HashMap;

// Problem: Course Schedule
// Tags: Depth-First Search, Breadth-First Search, Graph, Topological Sort
pub struct Solution {}
// dfs
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // Build the graph as an adjacency list
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..num_courses {
            adj_list.insert(i, vec![]);
        }
        for pre in prerequisites {
            adj_list.get_mut(&pre[1]).unwrap().push(pre[0]);
        }

        // Visited array: 0 = not visited, 1 = visiting, 2 = visited
        let mut visited = vec![0; num_courses as usize];

        // Perform DFS for each course
        for course in 0..num_courses {
            if !Solution::dfs(course, &adj_list, &mut visited) {
                return false; // If a cycle is detected, return false
            }
        }

        true // If no cycle is found, all courses can be finished
    }

    fn dfs(course: i32, adj_list: &HashMap<i32, Vec<i32>>, visited: &mut Vec<i32>) -> bool {
        let course_index = course as usize;

        // If the course is being visited, a cycle is detected
        if visited[course_index] == 1 {
            return false;
        }

        // If the course is fully processed, skip it
        if visited[course_index] == 2 {
            return true;
        }

        // Mark the course as being visited
        visited[course_index] = 1;

        // Visit all its neighbors
        for &neighbor in &adj_list[&course] {
            if !Solution::dfs(neighbor, adj_list, visited) {
                return false;
            }
        }

        // Mark the course as fully processed
        visited[course_index] = 2;

        true
    }
}

// brute-force logic
// impl Solution {
//     pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
//         // Create a hashmap to represent the graph
//         let mut connections: HashMap<i32, Vec<i32>> = HashMap::new();
//         for i in 0..num_courses {
//             connections.insert(i, vec![]);
//         }

//         // Fill the graph with prerequisites
//         for pre in &prerequisites {
//             let (unlock, req) = (pre[0], pre[1]);
//             connections.get_mut(&unlock).unwrap().push(req);
//         }

//         for kv in &connections {
//             println!("{:?}", kv);
//         }

//         // Initialize the queue with courses that have no prerequisites
//         let mut q: Vec<i32> = vec![];
//         for (k, v) in &connections {
//             if v.is_empty() {
//                 q.push(*k);
//             }
//         }

//         // Process the queue
//         while let Some(popped) = q.pop() {
//             for (k, v) in connections.iter_mut() {
//                 // Use a single loop to check and remove the element
//                 if let Some(pos) = v.iter().position(|&x| x == popped) {
//                     v.remove(pos); // Remove the prerequisite
//                     if v.is_empty() {
//                         q.push(*k); // Add to queue if no more prerequisites
//                     }
//                 }
//             }
//         }

//         // Check if all courses can be completed
//         connections.values().all(|v| v.is_empty())
//     }
// }
