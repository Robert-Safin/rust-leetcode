use std::collections::HashMap;

// Problem: Course Schedule II
// Tags: Depth-First Search, Breadth-First Search, Graph, Topological Sort
pub struct Solution {}
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut connections: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..num_courses {
            connections.insert(i, vec![]);
        }

        for pre in &prerequisites {
            let (unlock, req) = (pre[0], pre[1]);
            connections.get_mut(&unlock).unwrap().push(req);
        }

        let mut q: Vec<i32> = vec![];
        for (k, v) in &connections {
            if v.is_empty() {
                q.push(*k);
            }
        }

        let mut out = vec![];

        while let Some(popped) = q.pop() {
            out.push(popped);
            for (k, v) in connections.iter_mut() {
                if let Some(pos) = v.iter().position(|&x| x == popped) {
                    v.remove(pos);
                    if v.is_empty() {
                        q.push(*k);
                    }
                }
            }
        }
        if connections.values().all(|v| v.is_empty()) {
            return out;
        } else {
            return vec![];
        }
    }
}
