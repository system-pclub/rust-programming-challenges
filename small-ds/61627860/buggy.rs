/// assume this function can't be modified.
fn foo<A>(
    f1: impl Fn(&str) -> Result<(&str, A), ()>,
    base: &str,
    f2: impl Fn(A) -> bool
) {
    let s: String = base.to_owned();
    let option = Some(s.as_ref());
    let mapped = option.map(f1);
    let r = mapped.unwrap();
    let (rem, prod) = r.unwrap();
    assert!(f2(prod));
    assert_eq!(rem.len(), 0);
}


fn main() {
    fn bar<'a>(s: &'a str) -> Result<(&'a str, &'a str), ()> {
        Ok((&s[..1], &s[..]))
    }


    fn baz(s: &str) -> Result<(&str, &str), ()> {
        Ok((&s[..1], &s[..]))
    }

    foo(bar, "string", |s| s.len() == 5); // fails to compile

    foo(baz, "string", |s| s.len() == 5); // fails to compile 
}