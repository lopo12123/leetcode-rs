struct MyHashSet {
    inner: Vec<bool>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        MyHashSet { inner: vec![false; 1e6 as usize + 1] }
    }

    fn add(&mut self, key: i32) {
        self.inner[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.inner[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.inner[key as usize]
    }
}