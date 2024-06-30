#[derive(Debug)]
struct BST {
    value: i32,
    l: Option<Box<BST>>,
    r: Option<Box<BST>>,
}

impl BST {
    fn left_rotate(self) -> BST {
        /*
         *   (x)                 (y)
         *  /   \               /   \
         * a     (y)    =>   (x)     c
         *      /   \       /  \
         *     b     c     a    b
         */
        match self.r {
            None => self,
            Some(mut y) => BST {
                value: y.value,
                l: Some(Box::new(BST {
                    value: self.value,
                    l: self.l,
                    r: y.l.take(),
                })),
                r: y.r.take(),
            },
        }
    }

    fn build_three_node(a: i32, b: i32, c: i32) -> BST {
        BST {
            value: b,
            l: Some(Box::new(BST {
                value: a,
                l: None,
                r: None,
            })),
            r: Some(Box::new(BST {
                value: c,
                l: None,
                r: None,
            })),
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
