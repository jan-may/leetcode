/**
 * [145] Binary Tree Postorder Traversal
 *
 * Given the root of a binary tree, return the postorder traversal of its nodes' values.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/28/pre1.jpg" style="width: 127px; height: 200px;" />
 * Input: root = [1,null,2,3]
 * Output: [3,2,1]
 *
 * <strong class="example">Example 2:
 *
 * Input: root = []
 * Output: []
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [1]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	The number of the nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 *
 *  
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-postorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn postorder(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                postorder(&node.left, result);
                postorder(&node.right, result);
                result.push(node.val);
            }
        }

        let mut result = Vec::new();
        postorder(&root, &mut result);
        result
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_145() {}
}
