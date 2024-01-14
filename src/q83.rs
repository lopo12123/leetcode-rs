// Definition for singly-linked list.
#[allow(unused)]
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

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr = head.as_mut();

        while let Some(node) = ptr.take() {
            'next_diff: while let Some(next_node) = node.next.as_mut() {
                if next_node.val == node.val {
                    node.next = next_node.next.take();
                } else {
                    break 'next_diff;
                }
            }

            ptr = node.next.as_mut();
        }

        head
    }
}