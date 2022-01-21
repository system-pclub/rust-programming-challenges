struct Container<'a> {
    x: &'a i32,
}

trait Reply {}
impl Reply for i32 {}

fn json(_val: &Container) -> impl Reply {
    3
}

fn f() -> impl Reply {
    let i = 123;
    let a = Container { x: &i };
    json(&a)
}

fn main() {}