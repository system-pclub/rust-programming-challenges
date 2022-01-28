use lazy_static::lazy_static; // 1.3.0
use std::sync::Mutex;

struct Something;

lazy_static! {
    static ref SHARED: Mutex<Something> = Mutex::new(Something);
}

pub fn lock_and_execute(f: Box<Fn()>) {
    let _locked = SHARED.lock(); // `_locked` is never used.
                                 // does its lifetime end here?
    f();
}

fn main() {}