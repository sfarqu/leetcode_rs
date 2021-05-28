/**
https://leetcode.com/problems/maximum-depth-of-binary-tree/submissions/
Given the root of a binary tree, return its maximum depth.

A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
*/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
struct Solution;
impl Solution {
    // Recursive solution is cleanest and handles edge cases well
    // Visits each node once and returns maximum depth from both sub-trees, finishing with root node
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Use match to unwrap Option
        match root {
            Some(node_rc) => {
                // Strategy: get max depth from left & right subtrees & add 1 for current node

                // Borrow TreeNode out of the RefCell so we can access sub-trees
                // Assign to variable because we can only borrow once.
                let node = node_rc.borrow();
                // Using `clone` here is not expensive because we're cloning an Option
                // and the pointer inside it, not the underlying TreeNode.
                std::cmp::max(Solution::max_depth(node.left.clone()),
                              Solution::max_depth(node.right.clone()))
                    + 1
            },
            None => 0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Make it easier to write tree initializations (thanks Ninevra!)
    macro_rules! tree {
        ($value:expr,,) => {
            TreeNode {
                val: $value,
                left: None,
                right: None,
            }
        };
        ($value:expr, $left:tt,) => {
            TreeNode {
                val: $value,
                left: Some(Rc::new(RefCell::new(tree!$left))),
                right: None,
            }
        };
        ($value:expr, $left:tt, $right:tt) => {
            TreeNode {
                val: $value,
                left: Some(Rc::new(RefCell::new(tree!$left))),
                right: Some(Rc::new(RefCell::new(tree!$right))),
            }
        };
        ($value:expr,, $right: tt) => {
            TreeNode {
                val: $value,
                left: None,
                right: Some(Rc::new(RefCell::new(tree!$right))),
            }
        };
    }

    #[test]
    fn test_trees() {
        let _t: TreeNode = tree![1,,];
        let _t2: TreeNode = tree![1, [2,,], [3,,]];
    }

    #[test]
    fn test_max_depth() {
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(tree![1,,])))),
            1
        );
        assert_eq!(Solution::max_depth(None), 0);
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(tree![3,[9,,],[20,[15,,],[7,, ]]])))),
            3
        );
    }
}