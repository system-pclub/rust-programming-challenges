fn indexer(mut f: impl FnMut()) {
    f()
}

fn foo<'a, F>(a: &'a mut String, mut f: F)
where
    F: FnMut(&mut str),
{
    indexer(|| f(a.as_mut_str()));
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let mut s = String::from("hello");
        foo(&mut s, |s| {
            s.make_ascii_uppercase();
        });
        assert_eq!(s, "HELLO");
    }
}
