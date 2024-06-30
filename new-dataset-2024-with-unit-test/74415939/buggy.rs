struct Foo {
    x: i64,
}

impl Foo {
    fn get_index(&mut self) -> Option<&i64> {
        if self.x < 0 {
            self.x = 0;
            None
        } else {
            Some(&self.x)
        }
    }

    fn use_index(&self, index: &i64) -> String {
        format!("{}", *index)
    }
}

fn main() {
    let mut f = Foo { x: 12 };
    if let Some(index) = f.get_index() {
        println!("{}", f.use_index(index));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let mut f = Foo { x: 12 };
        if let Some(index) = f.get_index() {
            assert_eq!(f.use_index(index), "12");
        }
    }
}