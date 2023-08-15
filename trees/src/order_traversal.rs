#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

// the references in the result vector are valid for as long as the tree is
pub fn in_order_traversal<'a, T>(root: &'a Option<Box<TreeNode<T>>>, result: &mut Vec<&'a T>) {
    if let Some(node) = root {
        in_order_traversal(&node.left, result);
        result.push(&node.value);
        in_order_traversal(&node.right, result);
    }
}

pub fn pre_order_traversal<'a, T>(root: &'a Option<Box<TreeNode<T>>>, result: &mut Vec<&'a T>) {
    if let Some(node) = root {
        result.push(&node.value);
        pre_order_traversal(&node.left, result);
        pre_order_traversal(&node.right, result);
    }
}

pub fn post_order_traversal<'a, T>(root: &'a Option<Box<TreeNode<T>>>, result: &mut Vec<&'a T>) {
    if let Some(node) = root {
        post_order_traversal(&node.left, result);
        post_order_traversal(&node.right, result);
        result.push(&node.value);
    }
}
