struct BST<'a> {
    value: i32,
    l: Option<&'a BST<'a>>,
    r: Option<&'a BST<'a>>,
}

impl<'a> BST<'a> {
    fn left_rotate(self) -> BST<'a> {
        /*
         *   (x)                 (y)
         *  /   \               /   \
         * a     (y)    =>   (x)     c
         *      /   \       /  \
         *     b     c     a    b
         */
        match self.r {
            None => self,
            Some(y) => BST {
                l: Some(&BST { l: self.l, r: y.l }),
                r: y.r,
            },
        }
    }

    fn build_three_node(a: i32, b: i32, c: i32) -> BST {
        BST {
            value: b,
            l: Some(&BST {
                value: a,
                l: None,
                r: None,
            }),
            r: Some(&BST {
                value: c,
                l: None,
                r: None,
            }),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_rotate() {
        let mut root = BST::build_three_node(1, 2, 3);
        root = root.left_rotate();
        assert_eq!(root.value, 3);
        assert_eq!(root.l.as_ref().unwrap().value, 2);
        assert_eq!(root.l.as_ref().unwrap().l.as_ref().unwrap().value, 1);
    }
}
