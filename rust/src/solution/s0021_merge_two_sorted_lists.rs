/**
 * [21] Merge Two Sorted Lists
 *
 * You are given the heads of two sorted linked lists list1 and list2.
 * Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
 * Return the head of the merged linked list.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg" style="width: 662px; height: 302px;" />
 * Input: list1 = [1,2,4], list2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 *
 * <strong class="example">Example 2:
 *
 * Input: list1 = [], list2 = []
 * Output: []
 *
 * <strong class="example">Example 3:
 *
 * Input: list1 = [], list2 = [0]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in both lists is in the range [0, 50].
 * 	-100 <= Node.val <= 100
 * 	Both list1 and list2 are sorted in non-decreasing order.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/merge-two-sorted-lists/
// discuss: https://leetcode.com/problems/merge-two-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode::new(0)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21() {}
}
