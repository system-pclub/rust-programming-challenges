pub struct FooStruct<'a> {
    pub bars: Vec<&'a str>,
}

pub trait FooTrait {
    fn getBars(&self) -> &[&str];
}

impl<'a> FooTrait for FooStruct<'a> {
    fn getBars(&self) -> &[&str] {
        &self.bars // cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
    }
}

fn main() {}
