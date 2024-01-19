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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fixed = ListNode::new(0);
        let mut tail = &mut fixed;

        let mut ptr1 = list1.as_ref();
        let mut ptr2 = list2.as_ref();

        while ptr1.is_some() && ptr2.is_some() {
            // let n1 = ptr1.take().unwrap();
            let v1 = ptr1.unwrap().val;
            let v2 = ptr2.unwrap().val;

            if v1 < v2 {
                tail.next = Some(Box::new(ListNode::new(v1)));
                ptr1 = ptr1.unwrap().next.as_ref();
            } else {
                tail.next = Some(Box::new(ListNode::new(v2)));
                ptr2 = ptr2.unwrap().next.as_ref();
            }

            tail = tail.next.as_mut().unwrap();
        }

        if ptr1.is_some() {
            tail.next = Some(ptr1.unwrap().to_owned())
        } else if ptr2.is_some() {
            tail.next = Some(ptr2.unwrap().to_owned())
        }

        fixed.next
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        todo!()
    }
}