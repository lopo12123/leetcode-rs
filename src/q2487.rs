// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let head_inner = head.unwrap();

        // List of tuple represents (val, index)
        let mut stack = vec![];

        // walk through
        let mut ptr = &head_inner;
        let mut ctr = 0i32;
        loop {
            // push
            stack.push((ptr.val, ctr));

            // check
            if let Some(node) = &ptr.next {
                ptr = node;
                ctr += 1;
            } else {
                break;
            }
        };

        // do filter thing
        let mut max = i32::MIN;
        for i in (0..stack.len()).rev() {
            if stack[i].0 < max {
                stack[i].1 = -1;
            } else {
                max = stack[i].0;
            }
        }

        // re-chain TODO: 所有权问题
        // fixed head
        let mut new_head: Option<Box<ListNode>> = None;
        // last kept node
        let mut new_tail: Option<Box<ListNode>> = None;
        // pointer
        let mut ptr = &head_inner;
        // counter
        let mut ctr = 0i32;
        loop {
            // kept item
            if stack[ctr as usize].1 == ctr {
                match new_tail {
                    // re-assign head (init tail as well)
                    None => {
                        new_head = Some(ptr.to_owned());
                        new_tail = new_head.to_owned();
                    }
                    // re-assign next
                    Some(ref mut tail) => {
                        // tail.next = Some(ptr.to_owned());
                        let _ = std::mem::replace(&mut tail.next, Some(ptr.to_owned()));
                        new_tail = Some(ptr.to_owned());
                    }
                }
            }

            // stop signal
            ctr += 1;
            if ctr == stack.len() as i32 {
                break;
            }

            // forward
            ptr = ptr.as_ref().next.as_ref().unwrap();
        };

        new_head
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    fn list_builder(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(v[0])));

        let mut tail = head.as_mut().unwrap();
        for i in 1..v.len() {
            tail.next = Some(Box::new(ListNode::new(v[i])));
            tail = tail.next.as_mut().unwrap();
        }
        head
    }

    #[test]
    fn t() {
        let l1 = list_builder(vec![5, 2, 13, 3, 8]);
        let _l2 = list_builder(vec![1, 1, 1, 1, 1]);

        println!("{:#?}", Solution::remove_nodes(l1));
        // println!("{:?}", Solution::remove_nodes(l2));
    }
}