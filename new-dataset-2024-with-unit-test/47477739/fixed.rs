use std::rc::Rc;
use std::cell::RefCell;

struct Handler {
    list: Rc<RefCell<Vec<Rc<RefCell<Handler>>>>>,
}

impl Handler {
    fn new(list: Rc<RefCell<Vec<Rc<RefCell<Handler>>>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Handler { list }))
    }

    fn push(self_: &Rc<RefCell<Self>>) {
        self_.borrow_mut().list.borrow_mut().push(self_.clone());
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler() {
        let list = Rc::new(RefCell::new(Vec::new()));

        let mut h1 = Handler::new(list.clone());
        let mut h2 = Handler::new(list.clone());

        Handler::push(&mut h1);
        Handler::push(&mut h2);

        let mut cnt = 0;
        // Here the list should contain both h1 and h2
        for handler in list.borrow().iter() {
            cnt += 1;
        }
        assert_eq!(cnt, 2);
    }
}