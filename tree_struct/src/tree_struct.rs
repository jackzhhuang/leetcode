use std::{rc::Rc, cell::RefCell};

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
            continue;
        }
        let node = node_op.unwrap();
        if nums[index] == -99999 {
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
        if nums[index] == -99999 {
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

pub fn make_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
    let nodes = vec![Some(Rc::clone(&root))];
    make_sub_tree(nodes, nums, 1);
    Some(root)
}
