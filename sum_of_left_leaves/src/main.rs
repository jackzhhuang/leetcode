struct Solution;

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

fn make_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
    let nodes = vec![Some(Rc::clone(&root))];
    make_sub_tree(nodes, nums, 1);
    Some(root)
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn push_left_node(node: &Option<Rc<RefCell<TreeNode>>>, left: bool, left_node_values: &mut Vec<i32>) -> bool {
        match node {
            Some(n) => {
                if left {
                    // it is a left node
                    let l = Solution::push_left_node(&n.borrow().left, true, left_node_values);
                    let r = Solution::push_left_node(&n.borrow().right, false, left_node_values);
                    if l && r {
                        // the left node has None children
                        left_node_values.push(n.borrow().val);
                    }
                } else {
                    // it is a right node
                    Solution::push_left_node(&n.borrow().left, true, left_node_values);
                    Solution::push_left_node(&n.borrow().right, false, left_node_values);
                }

                return false;
            }
            None => true, // a None child
        }
    }
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut left_node_values = vec![];
        Solution::push_left_node(&root, false, &mut left_node_values);
        left_node_values.iter().sum()
    }
}

fn main() {
    let null = -99999;

    // let root = make_tree(vec![3,9,20,-99999,-99999,15,7]);
    // println!("{}", Solution::sum_of_left_leaves(root));

    // let root = make_tree(vec![1]);
    // println!("{}", Solution::sum_of_left_leaves(root));

    let root = make_tree(vec![3,4,5,-7,-6,null,null,-7,null,-5,null,null,null,-4]);
    println!("{}", Solution::sum_of_left_leaves(root));

}
