#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, directions: &[&str], val: i32) {
        let mut cur_node = self;

        let len = directions.len();
        let mut directions = directions.into_iter().copied();
        for direction in directions.by_ref().take(len - 1) {
            let child_node = match direction {
                "left" => &mut cur_node.left,
                "right" => &mut cur_node.right,
                _ => panic!("invalid direction {direction}"),
            };
            cur_node = child_node.as_mut().expect("invalid path").as_mut();
        }

        let new_node = Some(Box::new(TreeNode::new(val)));
        match directions.next().unwrap() {
            "left" => cur_node.left = new_node,
            "right" => cur_node.right = new_node,
            direction @ _ => panic!("invalid direction {direction}"),
        }
    }
}

fn task(values: &[i32; 4]) -> String {
    let mut tree = TreeNode::new(0);
    tree.insert(&["left"], values[0]);
    tree.insert(&["right"], values[1]);
    tree.insert(&["left", "left"], values[2]);
    tree.insert(&["right", "left"], values[3]);
    format!(
        "{} {} {} {}",
        tree.val,
        tree.left.as_ref().unwrap().val,
        tree.right.as_ref().unwrap().val,
        tree.left.as_ref().unwrap().left.as_ref().unwrap().val
    )
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let values = [1, 2, 2, 3];
        assert_eq!(task(&values), "0 1 2 2");
    }
}
