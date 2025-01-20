// Problem: Redundant Connection
// Tags: Depth-First Search, Breadth-First Search, Union Find, Graph

use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for edge in edges.iter() {
            let (u, v) = (edge[0], edge[1]);
            let mut visited = HashSet::new();
            if graph.contains_key(&u)
                && graph.contains_key(&v)
                && Solution::dfs(&graph, u, v, &mut visited)
            {
                return edge.clone();
            }

            graph.entry(u).or_insert(vec![]).push(v);
            graph.entry(v).or_insert(vec![]).push(u);
        }

        vec![]
    }
    fn dfs(graph: &HashMap<i32, Vec<i32>>, u: i32, v: i32, visited: &mut HashSet<i32>) -> bool {
        if u == v {
            return true;
        }
        visited.insert(u);
        if let Some(neighbors) = graph.get(&u) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) && Solution::dfs(graph, neighbor, v, visited) {
                    return true;
                }
            }
        }
        false
    }
}
