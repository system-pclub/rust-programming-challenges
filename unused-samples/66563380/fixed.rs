use std::{cell::RefCell, rc::Rc};

pub struct Node<T: PartialOrd> {
    key: T,
    par_right: Option<bool>,
    parent: Option<Rc<RefCell<Node<T>>>>,
    left_child: Option<Rc<RefCell<Node<T>>>>,
    right_child: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: PartialOrd> Node<T> {
    pub fn smallest(&self) -> &Self {
        match self.left_child.as_ref() {
            Some(lc) => {
                let t = lc.borrow().smallest();
                &*t
            }
            None => self,
        }
    }
}

fn main() {}
