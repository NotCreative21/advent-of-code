fn main() {
    println!("Hello, world!");
}

trait Solution {}

mod onehundred {
    use crate::Solution;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::cell::Ref;
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
    // lmao doesn't work properly
    impl dyn Solution {
        #[inline]
        fn traverse(l: Ref<'_, TreeNode>, r: Ref<'_, TreeNode>) -> bool {
            (match (l.left.as_ref(), r.left.as_ref()) {
                (Some(l), Some(r)) => return Self::traverse(l.borrow(), r.borrow()) && l.borrow().val == r.borrow().val,
                (None, None) => l.val == r.val,
                _ => false,
            }) && (match (l.right.as_ref(), r.right.as_ref()) {
                (Some(l), Some(r)) => return Self::traverse(l.borrow(), r.borrow()) && l.borrow().val == r.borrow().val,
                (None, None) => l.val == r.val,
                _ => false,
            })
        }
        pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (Some(q), Some(p)) => Self::traverse(p.borrow(), q.borrow()) && q.borrow().val == p.borrow().val,
                (None, None) => true,
                _ => false,
            }
        }
    }
}
