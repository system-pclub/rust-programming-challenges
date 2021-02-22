// no compilation errror

use core::ops::Deref;

#[derive(Debug)]
struct A<'b, 'c, T> {
    child_b: &'b T,
    child_c: &'c T
}

impl<'b, 'c, T> A<'b, 'c, T> {
    pub fn new_wrapper(b_value: &'b T, c_value: &'c T) -> A<'b, 'c, T> {
        A {
            child_b: b_value,
            child_c: c_value
        }
    }
}

fn doesnt_drop_borrow<T: Copy>(ty: &T) {
    *ty;
}

fn drop<T>(ty: T) { }

fn main() {
    let b: String = "wonderful".into();
    let c: String = "lifetime".into();
    let a = A::new_wrapper(&b, &c);
    println!("hello this {:?} world", &a);
    doesnt_drop_borrow(&a.child_c);
    drop(a.child_c);
    println!("hello this {:?} world", &a);
}
