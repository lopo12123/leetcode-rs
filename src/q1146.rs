struct SnapshotArray {
    snap_id: i32,
    // (snap_id, val)
    ops: Vec<Vec<(i32, i32)>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            snap_id: 0,
            ops: vec![vec![(-1, 0)]; length as usize],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.ops[index as usize].push((self.snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        return self.snap_id - 1;
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let his = &self.ops[index as usize];

        for &(op_id, val) in his.iter().rev() {
            if op_id <= snap_id {
                return val;
            }
        }
        return 0;
    }
}

#[cfg(test)]
#[test]
fn t() {
    let mut obj = SnapshotArray::new(2);
    obj.snap();
    println!("1");
    obj.set(1, 17);
    println!("1");
    obj.set(0, 20);
    obj.snap();
    println!("1");
    obj.snap();
    println!("1");
    obj.snap();
}