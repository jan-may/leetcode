/**
 * [938] Range Sum of BST
 *
 * Given the root node of a binary search tree and two integers low and high, return the sum of values of all nodes with a value in the inclusive range [low, high].
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/bst1.jpg" style="width: 400px; height: 222px;" />
 * Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
 * Output: 32
 * Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/bst2.jpg" style="width: 400px; height: 335px;" />
 * Input: root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
 * Output: 23
 * Explanation: Nodes 6, 7, and 10 are in the range [6, 10]. 6 + 7 + 10 = 23.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 2 * 10^4].
 * 	1 <= Node.val <= 10^5
 * 	1 <= low <= high <= 10^5
 * 	All Node.val are unique.
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/range-sum-of-bst/
// discuss: https://leetcode.com/problems/range-sum-of-bst/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut result = 0;
        Solution::range_sum_bst_helper(root, low, high, &mut result);
        result
    }

    fn range_sum_bst_helper(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, result: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val >= low && node.val <= high {
                *result += node.val;
            }
            if node.val > low {
                Solution::range_sum_bst_helper(node.left.clone(), low, high, result);
            }
            if node.val < high {
                Solution::range_sum_bst_helper(node.right.clone(), low, high, result);
            }}
}}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_938() {
    }
}
