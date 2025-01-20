// Problem: Reverse Linked List
// Tags: Linked List, Recursion
//
pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }
//
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr): (Option<Box<ListNode>>, Option<Box<ListNode>>) = (None, head);

        while let Some(mut node) = curr {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}
