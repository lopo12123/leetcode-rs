// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // a fake header with node value out of range
        let mut fixed = ListNode::new(-101);
        fixed.next = head;

        let mut ptr = &mut fixed;

        while ptr.next.is_some() && ptr.next.as_ref().unwrap().next.is_some() {
            if ptr.next.as_ref().unwrap().val == ptr.next.as_ref().unwrap().next.as_ref().unwrap().val {
                let mut next_node = ptr.next.as_ref().unwrap().next.as_ref();
                let v = ptr.next.as_ref().unwrap().val;
                while let Some(next_inner) = next_node.take() {
                    if next_inner.val == v {
                        next_node = next_inner.next.as_ref();
                    } else {
                        break;
                    }
                }
                ptr.next = Some(next_node.unwrap().to_owned());
            } else {
                ptr = ptr.next.as_mut().unwrap();
            }
        }

        fixed.next
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
        let list = list_builder(vec![1, 2, 3, 3, 4, 4, 5]);

        println!("{:?}", Solution::delete_duplicates(list));
    }
}