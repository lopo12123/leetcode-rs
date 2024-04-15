struct MyHashMap {
    inner: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        MyHashMap { inner: vec![-1; 1e6 as usize + 1] }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.inner[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.inner[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.inner[key as usize] = -1;
    }
}