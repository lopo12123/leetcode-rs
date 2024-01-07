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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut ptr_result = &mut result;

        let mut ex = false;
        let mut ptr1 = &l1;
        let mut ptr2 = &l2;
        while let (Some(node1), Some(node2)) = (ptr1, ptr2) {
            let v = node1.val + node2.val + if ex { 1 } else { 0 };
            ex = v > 9;
            ptr_result.next = Some(Box::new(ListNode::new(if ex { v % 10 } else { v })));

            ptr1 = &node1.next;
            ptr2 = &node2.next;
            ptr_result = ptr_result.next.as_mut().unwrap();
        };

        if !ex {
            ptr_result.next = ptr1.to_owned().or(ptr2.to_owned());
        } else {
            let mut ptr = ptr1.to_owned().or(ptr2.to_owned());
            while let Some(node) = ptr {
                let v = node.val + if ex { 1 } else { 0 };
                ex = v > 9;
                ptr_result.next = Some(Box::new(ListNode::new(if ex { v % 10 } else { v })));

                ptr = node.next;
                ptr_result = ptr_result.next.as_mut().unwrap();
            }
        }

        if ex {
            ptr_result.next = Some(Box::new(ListNode::new(1)));
        }

        result.next
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        // let l1 = Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode::new(3))) })) }));
        // let l2 = Some(Box::new(ListNode { val: 5, next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode::new(4))) })) }));
        let l1 = Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode::new(9))) })) }));
        let l2 = Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode::new(9))) }));

        println!("{:?}", Solution::add_two_numbers(l1, l2));
    }
}