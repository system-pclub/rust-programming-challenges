
struct TreeNode<'a> {
    left: Option<&'a TreeNode<'a>>,
    right: Option<&'a TreeNode<'a>>,
    sibling: Option<&'a TreeNode<'a>>,
    value: String,
}

fn build_tree_node<'a>(
    a_left: Option<&'a TreeNode<'a>>,
    a_right: Option<&'a TreeNode<'a>>,
    a_value: String,
) -> TreeNode<'a> {
    TreeNode {
        left: a_left,
        right: a_right,
        value: a_value,
        sibling: None,
    }
}

fn set_sibling(node: &mut Option<&TreeNode>) {
    match node {
        Some(mut n) => {
            match n.left {
                Some(mut c) => {
                    //c*.sibling = n.right;
                    match n.sibling {
                        Some(s) => n.right.unwrap().sibling = s.left,
                        None => {}
                    }
                }
                None => {}
            }
        }
        None => return,
    }
}

fn main() {}
