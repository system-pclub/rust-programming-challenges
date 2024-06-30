use std::cell::Cell;

#[derive(Debug)]
pub struct Entity<'a> {
    pub states: Vec<Cell<bool>>,
    pub subscribers: Vec<&'a Cell<bool>>,
}

impl<'a> Entity<'a> {
    pub fn new(size: usize) -> Self {
        Entity {
            states: vec![Cell::new(false); size],
            subscribers: Vec::new(),
        }
    }

    pub fn connect<'b: 'a>(publisher: &'a mut Entity<'a>, subscriber: &'b mut Entity<'b>) {
        for state in subscriber.states.iter() {
            publisher.subscribers.push(state);
        }
    }

    pub fn notify(&mut self) {
        for subscriber in self.subscribers.iter() {
            subscriber.replace(true);
        }
    }
}

fn main() {
    let mut pub_ = Entity::new(3);
    let mut sub_ = Entity::new(3);

    Entity::connect(&mut pub_, &mut sub_);

    pub_.notify();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity() {
        let mut pub_ = Entity::new(3);
        let mut sub_ = Entity::new(3);

        // view initial states of subscriber
        assert_eq!(
            sub_.states
                .iter()
                .map(|state| state.get())
                .collect::<Vec<_>>(),
            vec![false, false, false]
        );

        Entity::connect(&mut pub_, &mut sub_);

        // notify the subscriber (change all states)
        pub_.notify();

        // view final states of subscriber
        assert_eq!(
            sub_.states
                .iter()
                .map(|state| state.get())
                .collect::<Vec<_>>(),
            vec![true, true, true]
        );
    }
}
