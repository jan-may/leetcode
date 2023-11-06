/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [1,2,3,4]
 * Output: [2,1,4,3]
 *
 * <strong class="example">Example 2:
 *
 * Input: head = []
 * Output: []
 *
 * <strong class="example">Example 3:
 *
 * Input: head = [1]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 100].
 * 	0 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/swap-nodes-in-pairs/
// discuss: https://leetcode.com/problems/swap-nodes-in-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut dummy_ref = dummy.as_mut();
        let mut head_ref = head.as_ref();
        while let Some(node) = head_ref {
            let mut next_ref = node.next.as_ref();
            if let Some(next_node) = next_ref {
                dummy_ref.as_mut().unwrap().next = Some(Box::new(ListNode::new(next_node.val)));
                dummy_ref = dummy_ref.unwrap().next.as_mut();
                dummy_ref.as_mut().unwrap().next = Some(Box::new(ListNode::new(node.val)));
                dummy_ref = dummy_ref.unwrap().next.as_mut();
                head_ref = next_ref.unwrap().next.as_ref();
            } else {
                dummy_ref.as_mut().unwrap().next = Some(Box::new(ListNode::new(node.val)));
                dummy_ref = dummy_ref.unwrap().next.as_mut();
                head_ref = next_ref;
            }
        }
        dummy.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {}
}
