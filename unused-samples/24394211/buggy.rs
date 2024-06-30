enum BinaryTree<T> {
    Leaf(T),
    Branch(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
    Null,
}

fn create_binary_search_tree(vector: Vec<i32>) -> BinaryTree<i32> {
    fn insert_node(val: i32, btree: &BinaryTree<i32>) -> BinaryTree<i32> {
        match &btree {
            BinaryTree::Leaf(tval) if val > *tval => BinaryTree::Branch(
                *tval,
                Box::new(BinaryTree::Null),
                Box::new(BinaryTree::Leaf(val)),
            ),
            BinaryTree::Leaf(tval) if val < *tval => BinaryTree::Branch(
                *tval,
                Box::new(BinaryTree::Leaf(val)),
                Box::new(BinaryTree::Null),
            ),
            BinaryTree::Branch(tval, left, right) if val > *tval => {
                insert_node(val, right)
            }
            BinaryTree::Branch(tval, left, right) if val < *tval => {
                insert_node(val, left)
            }
            BinaryTree::Null => BinaryTree::Leaf(val),
            BinaryTree::Leaf(lval) if val == *lval => BinaryTree::Leaf(val),
            BinaryTree::Branch(lval, _, _) if val == *lval => {
                panic!("already has a node with {}", lval)
            }
            _ => BinaryTree::Null,
        }
    }

    let mut tree = BinaryTree::Null;
    for &v in &vector {
        tree = insert_node(v, &tree);
    }

    tree
}

fn print_tree(tree: &BinaryTree<i32>) {
    fn inner_print(prefix: &str, tree: &BinaryTree<i32>, level: i32) {
        let lv_desc = format!("lv {}", level);
        match tree {
            BinaryTree::Leaf(val) => println!("{}-{} leaf: {}", lv_desc, prefix, val),
            BinaryTree::Branch(val, ref left, ref right) => {
                println!("{}-{} node: {}", lv_desc, prefix, val);
                inner_print("left branch <-", left, level + 1);
                inner_print("right branch ->", right, level + 1);
            }
            BinaryTree::Null => println!("end"),
        }
    }
    inner_print("root", tree, 0);
}

fn main() {
    let vector = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let tree = create_binary_search_tree(vector);
    print_tree(&tree);
}
