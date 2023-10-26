/**
 * [652] Find Duplicate Subtrees
 *
 * Given the root of a binary tree, return all duplicate subtrees.
 * For each kind of duplicate subtrees, you only need to return the root node of any one of them.
 * Two trees are duplicate if they have the same structure with the same node values.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e1.jpg" style="width: 450px; height: 354px;" />
 * Input: root = [1,2,3,4,null,2,4,null,null,4]
 * Output: [[2,4],[4]]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e2.jpg" style="width: 321px; height: 201px;" />
 * Input: root = [2,1,1]
 * Output: [[1]]
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e33.jpg" style="width: 450px; height: 303px;" />
 * Input: root = [2,2,2,3,null,3,null]
 * Output: [[2,3],[3]]
 *
 *  
 * Constraints:
 *
 * 	The number of the nodes in the tree will be in the range [1, 5000]
 * 	-200 <= Node.val <= 200
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/find-duplicate-subtrees/
// discuss: https://leetcode.com/problems/find-duplicate-subtrees/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut map = std::collections::HashMap::new();
        let mut result = vec![];
        Solution::traverse(&root, &mut map, &mut result);
        result
    }

    fn traverse(
        root: &Option<Rc<RefCell<TreeNode>>>,
        map: &mut std::collections::HashMap<String, i32>,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> String {
        if let Some(node) = root {
            let left = Solution::traverse(&node.borrow().left, map, result);
            let right = Solution::traverse(&node.borrow().right, map, result);
            let key = format!("{} {} {}", node.borrow().val, left, right);
            let count = map.entry(key.clone()).or_insert(0);
            if *count == 1 {
                result.push(root.clone());
            }
            *count += 1;
            key
        } else {
            "#".to_owned()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_652() {}
}
