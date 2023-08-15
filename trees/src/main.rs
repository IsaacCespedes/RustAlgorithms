mod binary_heap;

mod binary_search;
use binary_heap::MinHeap;

mod order_traversal;
use order_traversal::{in_order_traversal, post_order_traversal, pre_order_traversal, TreeNode};
fn main() {
    let numbers = vec![11, 12, 22, 25, 34, 64, 90];
    let target = 25;

    // binary search
    println!("Binary Search");
    if let Some(index) = binary_search::binary_search(&numbers, &target) {
        println!("Target {} found at index {}", target, index);
    } else {
        println!("Target {} not found", target);
    }

    // min heap
    println!("Min Heap");
    let mut min_heap = MinHeap::new();

    min_heap.push(5);
    min_heap.push(10);
    min_heap.push(3);
    min_heap.push(8);
    min_heap.push(1);

    while let Some(min_value) = min_heap.pop() {
        println!("{}", min_value);
    }

    // order traversal
    let mut root: Option<Box<TreeNode<i32>>> = Some(Box::new(TreeNode::new(1)));
    root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
    root.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));

    // 1
    // 2 | 3
    // 4 5 |

    let mut in_order_result = Vec::new();
    in_order_traversal(&root, &mut in_order_result);
    println!("In-order traversal: {:?}", in_order_result);

    let mut pre_order_result = Vec::new();
    pre_order_traversal(&root, &mut pre_order_result);
    println!("Pre-order traversal: {:?}", pre_order_result);

    let mut post_order_result = Vec::new();
    post_order_traversal(&root, &mut post_order_result);
    println!("Post-order traversal: {:?}", post_order_result);
}
