struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        for _ in 0..self.stack2.len() {
            self.stack1.push(self.stack2.pop().unwrap());
        }
        self.stack1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.stack1.is_empty() {
            self.stack2.pop().unwrap()
        } else {
            for _ in 0..self.stack1.len() - 1 {
                self.stack2.push(self.stack1.pop().unwrap());
            }
            self.stack1.pop().unwrap()
        }
    }

    fn peek(&mut self) -> i32 {
        for _ in 0..self.stack1.len() {
            self.stack2.push(self.stack1.pop().unwrap());
        }
        *self.stack2.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        let ret_2: i32 = obj.peek();  // 1
        let ret_3: i32 = obj.pop();  // 1
        let ret_4: bool = obj.empty();  // false

        println!("ret_2: {}, ret_3: {}, ret_4: {}", ret_2, ret_3, ret_4)
    }
}