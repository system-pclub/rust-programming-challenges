#[macro_use]
extern crate lazy_static;

fn accept(input: &[String]) {
    let vec = if input.is_empty() {
        lazy_static! {
            static ref DEFAULT: Vec<String> = vec!["empty".to_string()];
        }
        &DEFAULT
    } else {
        input
    };
    // ... do something with `vec`, like looping over it
}

fn main() {}