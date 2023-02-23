struct Solution {
}

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
impl Solution {
    fn symmetric_nodes(left_op: &Option<Rc<RefCell<TreeNode>>>, right_op: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left_op, right_op) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                return left.borrow().val == right.borrow().val &&
                Solution::symmetric_nodes(&left.borrow().left, &right.borrow().right) &&
                Solution::symmetric_nodes(&left.borrow().right, &right.borrow().left);
            }
            _ => false
        }
    }

    pub fn is_symmetric(root_op: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root_op {
            None => true,
            Some(root) => {
                return Solution::symmetric_nodes(&root.borrow().left, &root.borrow().right);
            }
        }
    }
}

fn make_sub_tree(nodes: Vec<Option<Rc<RefCell<TreeNode>>>>, nums: Vec<i32>, mut index: usize) {
    if index >= nums.len() {
        return;
    }
    let mut new_nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
    for node_op in nodes {
        if index >= nums.len() {
            break;
        }
        if let None = node_op {
            new_nodes.push(None);
            index += 1;
            continue;
        }
        let node = node_op.unwrap();
        if nums[index] == -999 {
            new_nodes.push(None);
        } else {
            let left = Rc::new(RefCell::new(TreeNode::new(nums[index])));
            node.as_ref().borrow_mut().left = Some(Rc::clone(&left));
            new_nodes.push(Some(Rc::clone(&left)));
        }
        index += 1;

        if index >= nums.len() {
            break;
        }
        if nums[index] == -999 {
            new_nodes.push(None);
        } else {
            let right = Rc::new(RefCell::new(TreeNode::new(nums[index])));
            node.as_ref().borrow_mut().right = Some(Rc::clone(&right));
            new_nodes.push(Some(Rc::clone(&right)));
        }
        index += 1;
    }
    if new_nodes.len() != 0 {
        make_sub_tree(new_nodes, nums, index);
    }
}

fn make_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
    let nodes = vec![Some(Rc::clone(&root))];
    make_sub_tree(nodes, nums, 1);
    Some(root)
}

fn test(nums: &[i32]) {
    let root = make_tree(nums.into());
    let result = Solution::is_symmetric(root);
    if result {
        println!("is symmetric");
    } else {
        println!("is not symmetric");
    }
}

fn main() {
    test(&[1, 2, 2, 3, 4, 4, 3]);
    test(&[1, 2, 2, -999, 3, -999, 3]);
    test(&[1, 2, 3]);
    test(&[1,2,2,-999,3,3]);
}

