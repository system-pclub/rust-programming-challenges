#[derive(Debug)]
struct MyPtr<'a> {
    x: &'a usize,
}

#[derive(Debug)]
struct MyStruct {
    data: Vec<usize>,
}
trait MyTrait<'a> {
    type N: 'a + std::fmt::Debug;

    fn func(&'a self) -> Self::N;
}

impl<'a> MyTrait<'a> for MyStruct {
    type N = MyPtr<'a>;

    fn func(&'a self) -> Self::N {
        MyPtr { x: &self.data[0] }
    }
}

fn myfunc<T: for<'a> MyTrait<'a>>(g: T) -> String {
    format!("{:?}", g.func())
}

fn main() {
    let s = MyStruct { data: vec![100, 101] };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = MyStruct { data: vec![100, 101] };
        assert_eq!(myfunc(s), "MyPtr { x: 100 }".to_string());
    }
}