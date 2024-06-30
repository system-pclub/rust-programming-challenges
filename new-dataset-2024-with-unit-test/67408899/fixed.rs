#[derive(Debug)]
pub struct Turtle<'a> {
    children: Vec<&'a Turtle<'a>>,
}

impl<'a> Turtle<'a> {
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    pub fn add_child(&mut self, t: &'a Turtle<'a>) {
        self.children.push(t);
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turtle() {
        let mut t1 = Turtle::new();
        let t2 = Turtle::new();
        t1.add_child(&t2);
        assert_eq!(t1.children.len(), 1);
    }
}