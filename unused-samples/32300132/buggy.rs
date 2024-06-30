struct Parent {
    count: u32,
}

struct Child<'a> {
    parent: &'a Parent,
}

struct Combined<'a> {
    parent: Parent,
    child: Child<'a>,
}

impl<'a> Combined<'a> {
    fn new() -> Self {
        let parent = Parent { count: 42 };
        let child = Child { parent: &parent };

        Combined { parent, child }
    }
}

fn main() {}