// Problem: Min Stack
// Tags: Stack, Design

struct MinStack {
    stack: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![],
            mins: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);

        if self.mins.len() == 0 {
            self.mins.push(val);
        } else if self.mins.last().unwrap() >= &val {
            self.mins.push(val);
        }
    }

    fn pop(&mut self) {
        let popped = self.stack.pop().unwrap();
        if let Some(last) = self.mins.last() {
            if *last == popped {
                self.mins.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}
