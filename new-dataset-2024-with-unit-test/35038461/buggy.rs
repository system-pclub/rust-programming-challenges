trait MyTrait<'a> {
    type N: 'a + std::fmt::Debug;

    fn func(&'a self) -> Self::N;
}

fn myfunc<'a, T: 'a + MyTrait<'a>>(g: T) -> String {
    format!("{:?}", g.func())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    impl<'a> MyTrait<'a> for i32 {
        type N = i32;

        fn func(&'a self) -> i32 {
            *self
        }
    }

    #[test]
    fn test() {
        assert_eq!(myfunc(0), "0".to_string());
    }
}