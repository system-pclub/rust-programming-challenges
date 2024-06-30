#[derive(Default)]
pub struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn print_data(&self) -> i32 {
        self.data
    }

    fn new(data: i32) -> Node {
        //function to help when instantiate node with no children
        Node {
            data,
            left: None,
            right: None,
        }
    }

    fn set_left(&mut self, new_left: Option<Box<Node>>) -> () {
        self.left = new_left
    }
    fn set_right(&mut self, new_right: Option<Box<Node>>) -> () {
        self.right = new_right
    }

    fn swap_childs(&mut self) -> () {
        let tmpr = Some(self.right.unwrap());
        let tmpl = Some(self.left.unwrap());
        let nilr = self.right.take();
        let nill = self.left.take();
        self.right = tmpl;
        self.left = tmpr;
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_childs() {
        let mut root = Node::new(1);
        let left_child = Some(Box::new(Node::new(2)));
        let right_child = Some(Box::new(Node::new(3)));

        root.set_left(left_child);
        root.set_right(right_child);

        root.swap_childs();

        assert_eq!(root.left.unwrap().data, 3);
        assert_eq!(root.right.unwrap().data, 2);
    }
}
