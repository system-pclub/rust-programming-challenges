// Workaround code: the state of the function is placed 
// into a struct so that all of the references are valid
// for the lifetime of of &self. This way concrete lifetimes
// can be used because all lifetimes start on the function 
// call, rather than several statements into the function. 
//
// This work around eliminates the need for Higher Ranked 
// Trait Bounds (HRTBs), since all lifetimes are instantiated
// on function initialization.

struct Wrapper<'a>(&'a str);

struct ParserContext {
    inner: String
}

impl ParserContext {
    fn new(base: &str) -> Self {Self {inner: base.to_owned()}}

    fn call<'a, A>(
        &'a self, 
        f1: fn(&'a str) -> (&'a str, A),
        f2: fn(A) -> bool,
    ) {
        let (remaining, produced) = f1(self.inner.as_str());
        assert_eq!(remaining.len(), 0);
        assert!(f2(produced));
    }
}


fn main() {
    fn bar(s: &str) -> (&str, &str) {
        (&s[..0], &s[..])
    }

    fn baz(s: &str) -> (&str, Wrapper) {
        (&s[..0], Wrapper(&s[..]))
    }


    // I tried to extract the section below into its
    // own function previously (foo) but this would
    // always inevitably fail because of lifetime issues.
    // this work around only adds one line of code to the calling
    // function to create the context to hold the state 
    // used by the functions passed to `call`, so this 
    // is an acceptable, and rather eloquent work around.
    let pc = ParserContext::new("string");

    pc.call(bar, |s| s.len() == 6);
    pc.call(baz, |s| s.0.len() == 6);
}