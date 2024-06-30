#[derive(Clone, Debug)]
struct TreeNode {
    id: i32,
    children: Vec<TreeNode>,
}

// Returns a mutable reference to the parent of the node that matches the given id.
fn find_parent_mut<'a>(root: &'a mut TreeNode, id: i32) -> Option<&'a mut TreeNode> {
    for child in root.children.iter_mut() {
        if child.id == id {
            return Some(root);
        } else {
            let descendent_result = find_parent_mut(child, id);
            if descendent_result.is_some() {
                return descendent_result;
            }
        }
    }
    None
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tree = TreeNode {
            id: 1,
            children: vec![TreeNode {
                id: 2,
                children: vec![TreeNode {
                    id: 3,
                    children: vec![],
                }],
            }],
        };
        let a: Option<&mut TreeNode> = find_parent_mut(&mut tree, 3);
        assert_eq!(a.unwrap().id, 2);
    }
}
