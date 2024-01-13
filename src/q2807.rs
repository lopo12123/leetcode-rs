// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
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
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Solution::gcd(b, a % b) }
    }

    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head.as_mut().unwrap();

        while curr.next.is_some() {
            let next = curr.next.take();
            curr.next = Some(Box::new(ListNode {
                val: Solution::gcd(curr.val, next.as_ref().unwrap().val),
                next,
            }));
            curr = curr.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        head
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn t() {
        let chain = ListNode { val: 18, next: Some(Box::new(ListNode { val: 6, next: None })) };

        println!("{:?}", Solution::insert_greatest_common_divisors(Some(Box::new(chain))));
    }
}