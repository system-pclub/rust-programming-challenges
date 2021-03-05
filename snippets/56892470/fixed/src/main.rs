use std::rc::Rc;
use owning_ref::RcRef;

fn build_my_struct<'a>(s: &Rc<String>) -> RcRef<String, str> {
    let rc_string = RcRef::new(s.clone());
    let rc_string = rc_string.map(|s| &s[0..2]);
    rc_string
}

fn main() {}