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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Strategy: get all values from tree into a sorted array, then build a new tree
        // Use recursive functions for destructuring and recreating tree.
        let list = Solution::get_list(root, vec![]);
        Solution::build_tree(&list)
    }

    /**
    * This implementation works, but handles the `Vec<i32>` in a weird way that is maybe not the best.
    * The function signature accepts a *moved* value, and assigns it to the mutable internal variable
    * `list`. The value passed in does not need to be stored in a mutable variable, but because it is
    * moved, it can't be used after this function call. (Vec does not implement Copy so value is not
    * silently copied like it would be for primitive types.)
    *
    * As a result, we are not mutating the argument. Instead we create a new list, modify it, create another
    * new list, and then return that.

    * Using a mutable borrow (`list: &mut Vec<i32>`) might perform slightly better?
    */
    fn get_list(root: Option<Rc<RefCell<TreeNode>>>, mut list: Vec<i32>) -> Vec<i32> {
        match root {
            Some(node_rc) => {
                let node = node_rc.borrow();
                // Input value is already sorted, so just need to append left subtree, current node,
                // and right subtree in order
                list = Solution::get_list(node.left.clone(), list);
                list.push(node.val);
                // can assign to list again, but also works to just return the Vec after adding right subtree
                Solution::get_list(node.right.clone(), list)
            },
            None => list
        }
    }

    /**
    * Alternate implementation to create a Vec by mutating parameter in place
    */
    fn get_list_alt(root: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        // Since we do nothing if Option is None, use `if let` instead of `match`
        if let Some(node_rc) = root {
            let node = node_rc.borrow();
            // Input value is already sorted, so just need to append left subtree, current node,
            // and right subtree in order
            Solution::get_list_alt(node.left.clone(), list);
            list.push(node.val);
            Solution::get_list_alt(node.right.clone(), list);
        }
    }

    fn build_tree(list: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match list.len() {
            0 => None,
            _ => {
                // don't have to round because integer division always truncates to integer
                let mid = list.len() / 2;
                // wrap return value in Option<Rc<RefCell>> as expected
                Some(Rc::new(RefCell::new(
                    TreeNode{
                        // This solution is biased toward the left side of the tree, because `mid` is
                        // calculated based on len, not slice indexes. `mid` will be the middle element
                        // if len is odd, or the element above the middle if len is even.
                        val: list[mid],
                        // Rust ranges by default go up to but not including final value. Inclusive range
                        // would add duplicate of mid value.
                        left: Solution::build_tree(&list[..mid]),
                        // mid+1 only works on len == 1 because Rust handles slice indexes *up to and including* len(),
                        // even though in most languages values over len - 1 will crash.
                        // Example: if list == [1]
                        //   left: list[0..0] -> returns an empty list because start == end
                        //   right: list[1..1] -> does not crash because "1" is a valid index (returns empty value)
                        //                     -> returns empty list because start == end
                        right: Solution::build_tree(&list[mid+1..])
                    }
                )))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Make it easier to write tree initializations
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
    fn basic_test() {
        let tree = Some(Rc::new(RefCell::new(tree![1,,[2,,[3,,[4,,]]]])));
        let res = Some(Rc::new(RefCell::new(tree![3,[2,[1,,],],[4,,]])));

        assert_eq!(res,
                   Solution::balance_bst(tree));
    }


    #[test]
    fn single_node_test() {
        let tree = Some(Rc::new(RefCell::new(tree![1,,])));
        let res = Some(Rc::new(RefCell::new(tree![1,,])));

        assert_eq!(res,
                   Solution::balance_bst(tree));
    }
}