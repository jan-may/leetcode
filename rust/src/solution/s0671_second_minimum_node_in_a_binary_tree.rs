/**
 * [671] Second Minimum Node In a Binary Tree
 *
 * Given a non-empty special binary tree consisting of nodes with the non-negative value, where each node in this tree has exactly two or zero sub-node. If the node has two sub-nodes, then this node's value is the smaller value among its two sub-nodes. More formally, the property root.val = min(root.left.val, root.right.val) always holds.
 * Given such a binary tree, you need to output the second minimum value in the set made of all the nodes' value in the whole tree.
 * If no such second minimum value exists, output -1 instead.
 *  
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/15/smbt1.jpg" style="width: 431px; height: 302px;" />
 * Input: root = [2,2,5,null,null,5,7]
 * Output: 5
 * Explanation: The smallest value is 2, the second smallest value is 5.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/15/smbt2.jpg" style="width: 321px; height: 182px;" />
 * Input: root = [2,2,2]
 * Output: -1
 * Explanation: The smallest value is 2, but there isn't any second smallest value.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 25].
 * 	1 <= Node.val <= 2^31 - 1
 * 	root.val == min(root.left.val, root.right.val) for each internal node of the tree.
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/
// discuss: https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min = i32::MAX;
        let mut second_min = i32::MAX;
        let mut max_flag = false;
        Solution::walk(root, &mut min, &mut second_min, &mut max_flag);
        println!("{} {}", min, second_min);
        if min == second_min || second_min == i32::MAX && !max_flag {
            return -1
        }
        second_min
    }


fn walk(root: Option<Rc<RefCell<TreeNode>>>, min: &mut i32, second_min: &mut i32, max_flag: &mut bool) {
    if let Some(node) = root {
        let node = node.borrow();
        if node.val == i32::MAX {
            *max_flag = true;
        }
        if node.val < *min {
            *second_min = *min;
            *min = node.val;
        } else if node.val > *min && node.val < *second_min {
            *second_min = node.val;
        }
        Solution::walk(node.left.clone(), min, second_min, max_flag);
        Solution::walk(node.right.clone(), min, second_min, max_flag);
    }
}}


// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_671() {
    }
}
