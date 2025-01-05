// Problem: Valid Sudoku
// Tags: Array, Hash table, Matrix

use std::collections::{HashMap, HashSet};

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row_hash: HashMap<usize, Vec<char>> = HashMap::new();
    let mut col_hash: HashMap<usize, Vec<char>> = HashMap::new();
    let mut box_hash: HashMap<(usize, usize), Vec<char>> = HashMap::new();

    for (y, line) in board.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if let Some(v) = row_hash.get_mut(&y) {
                v.push(*c);
            } else {
                row_hash.insert(y, vec![*c]);
            }

            if let Some(v) = col_hash.get_mut(&x) {
                v.push(*c);
            } else {
                col_hash.insert(x, vec![*c]);
            }

            let box_x = x / 3;
            let box_y = y / 3;
            if let Some(v) = box_hash.get_mut(&(box_y, box_x)) {
                v.push(*c);
            } else {
                box_hash.insert((box_y, box_x), vec![*c]);
            }
        }
    }

    let mut set: HashSet<char> = HashSet::new();

    for (_, v) in row_hash.iter() {
        for c in v.iter() {
            if *c != '.' {
                if set.contains(c) {
                    return false;
                }
                set.insert(*c);
            }
        }
        set.clear();
    }

    for (_, v) in col_hash.iter() {
        for c in v.iter() {
            if *c != '.' {
                if set.contains(c) {
                    return false;
                }
                set.insert(*c);
            }
        }
        set.clear();
    }

    for (_, v) in box_hash.iter() {
        for c in v.iter() {
            if *c != '.' {
                if set.contains(c) {
                    return false;
                }
                set.insert(*c);
            }
        }
        set.clear();
    }

    true
}
