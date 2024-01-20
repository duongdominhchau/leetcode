pub struct MyQueue {
    s: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self { s: Vec::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.s.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        let mut temp_stack = Vec::new();
        while self.s.len() > 1 {
            temp_stack.push(self.s.pop().unwrap());
        }
        let value = self.s.pop().unwrap();
        while !temp_stack.is_empty() {
            self.s.push(temp_stack.pop().unwrap());
        }
        value
    }

    pub fn peek(&mut self) -> i32 {
        let mut temp_stack = Vec::new();
        while self.s.len() > 1 {
            temp_stack.push(self.s.pop().unwrap());
        }
        let value = self.s[0];
        while !temp_stack.is_empty() {
            self.s.push(temp_stack.pop().unwrap());
        }
        value
    }

    pub fn empty(&mut self) -> bool {
        self.s.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut queue = MyQueue::new();
        queue.push(1);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.empty(), false);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.empty(), true);
    }
}
