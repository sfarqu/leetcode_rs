/**
https://leetcode.com/problems/middle-of-the-linked-list/
Given a non-empty, singly linked list with head node `head`, return a middle node of linked list.

If there are two middle nodes, return the second middle node.

  * The number of nodes in the given list will be between 1 and 100.
*/

// Bare minimum definition for linked list, as specified in leetcode problem
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow_pointer = &head;
    let mut fast_pointer = &head;

    // while fast_pointer is not None
    while fast_pointer.is_some() && fast_pointer.as_ref().unwrap().next.is_some() {
        // slow_pointer = next
        slow_pointer = &slow_pointer.as_ref().unwrap().next;
        // fast_pointer = fast_pointer.next.next
        fast_pointer = &fast_pointer.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow_pointer.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // This seems like a really gross way to create a linked list, but I think it's inevitable
        // with the minimal "linked list" definition provided, and I don't feel like designing a
        // better one just for this test.
        let (mut n1, mut n2, mut n3, mut n4, n5) =
            (ListNode::new(1),ListNode::new(2),ListNode::new(3), ListNode::new(4), ListNode::new(5));
        n4.next = Some(Box::new(n5));
        n3.next = Some(Box::new(n4));
        let res = n3.clone(); // hack to get a result value I can use later without borrow checker complaining
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));
        assert_eq!(middle_node(Some(Box::new(n1))),Some(Box::new(res)));
    }
}