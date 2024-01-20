// https://leetcode.com/problems/implement-stack-using-queues/

use std::collections::VecDeque;

pub struct MyStack {
    q: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self { q: VecDeque::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.q.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        let mut new_q = VecDeque::new();
        while self.q.len() > 1 {
            new_q.push_back(self.q.pop_front().unwrap());
        }
        let t = self.q.pop_front().unwrap();
        self.q = new_q;
        t
    }

    pub fn top(&mut self) -> i32 {
        let val = self.pop();
        self.q.push_back(val);
        val
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut stack = MyStack::new();
        stack.push(1);
        assert_eq!(stack.top(), 1);
        assert_eq!(stack.empty(), false);
        assert_eq!(stack.pop(), 1);
        assert_eq!(stack.empty(), true);
    }
}
