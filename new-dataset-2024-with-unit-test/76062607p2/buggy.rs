struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
impl TreeNode {
    fn new(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> TreeNode {
        TreeNode { val, left, right }
    }
}

fn pre_order(root: Option<Box<TreeNode>>, output: &mut Vec<i32>) {
    match root {
        None => {
            return;
        }
        Some(root_node) => {
            output.push(root_node.val);
            pre_order(root_node.left, output);
            pre_order(root_node.right, output);
        }
    }
}
fn task() -> Vec<i32> {
    let root = Some(Box::new(TreeNode::new(
        120,
        Some(Box::new(TreeNode::new(
            150,
            Some(Box::new(TreeNode::new(180, None, None))),
            Some(Box::new(TreeNode::new(40, None, None))),
        ))),
        Some(Box::new(TreeNode::new(
            110,
            Some(Box::new(TreeNode::new(144, None, None))),
            None,
        ))),
    )));
    let mut output = Vec::new();
    pre_order(root, &mut output);
    pre_order(root, &mut output);
    output
}

fn main() {}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_preorder() {
        let ret = task();
        assert_eq!(
            &ret,
            &[120, 150, 180, 40, 110, 144, 120, 150, 180, 40, 110, 144]
        );
    }
}
