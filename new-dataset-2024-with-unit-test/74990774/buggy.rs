struct B<'a> {
    data: &'a mut &'a str,
}

pub fn task(hello: &str, world: &str) -> String {
    let mut s = hello;
    let a = B {
        data: &mut s,
    };
    *a.data = world;
    s.to_string()
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(task("hello", "world"), "world");
    }
}