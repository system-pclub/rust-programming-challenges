pub trait Action {
    fn execute(&self);
}

struct RpnCalculator;

struct AddAction<'a> {
    rpn_calculator: &'a RpnCalculator
}

struct DeductAction<'a> {
    rpn_calculator: &'a RpnCalculator
}

impl Action for DeductAction<'_> {
    fn execute(&self) {
        // ...
    }
}

impl Action for AddAction<'_> {
    fn execute(&self) {
        // ...
    }
}

impl<'a> RpnCalculator {
    fn actions(&self) -> Vec<Box<dyn Action + 'a>> {
        let mut actions: Vec<Box<dyn Action + 'a>> = vec![
            Box::new(AddAction { rpn_calculator: &self }),
            Box::new(AddAction { rpn_calculator: &self })
            // ...
        ];
        // ...
        actions
    }
}

fn main() {}