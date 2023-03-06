use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

///////// begin: make a tree /////////////
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
///////// end: make a tree /////////////


struct Solution;
impl Solution {
    fn depth(node: Rc<RefCell<TreeNode>>, max_len: &mut i32) -> i32 {
        let left_depth = if let Some(left_child) = &node.as_ref().borrow().left {
            Solution::depth(Rc::clone(left_child), max_len)
        } else {
            0
        };
        let right_depth = if let Some(right_child) = &node.as_ref().borrow().right {
            Solution::depth(Rc::clone(right_child), max_len)
        } else {
            0
        };

        *max_len = (*max_len).max(left_depth + right_depth);

        left_depth.max(right_depth) + 1
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_len: i32 = 0;
        match root {
            Some(node) => {
                let left_depth = if let Some(left_node) = &node.as_ref().borrow().left {
                    Solution::depth(Rc::clone(left_node), &mut max_len)
                } else {
                    0
                };
                let right_depth = if let Some(right_node) = &node.as_ref().borrow().right {
                    Solution::depth(Rc::clone(right_node), &mut max_len)
                } else {
                    0
                };

                max_len = max_len.max(left_depth + right_depth);
                max_len
            }
            None => 0,
        }
    }
}

fn main() {
    let root = make_tree(vec![1, 2, 3, 4, 5]);
    println!("{}", Solution::diameter_of_binary_tree(root));

    let root = make_tree(vec![1, 2]);
    println!("{}", Solution::diameter_of_binary_tree(root));

    let root = make_tree(vec![1]);
    println!("{}", Solution::diameter_of_binary_tree(root));

    let root = make_tree(vec![3, 1, 2]);
    println!("{}", Solution::diameter_of_binary_tree(root));
}
