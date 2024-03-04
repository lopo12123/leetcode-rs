struct MyStack {
    inner: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack { inner: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.inner.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.inner.pop().unwrap()
    }

    fn top(&self) -> i32 {
        *self.inner.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.inner.is_empty()
    }
}