use std::rc::Rc;

struct MyStruct<'a> {
    rc_string: Rc<String>,
    vec: Vec<&'a str>
}

fn build_my_struct<'a>(s: &Rc<String>) -> MyStruct<'a> {
    let rc_string = s.clone();
    let mut vec = Vec::new();
    vec.push(&rc_string[0..2]);

    MyStruct {
        rc_string: rc_string,
        vec: vec
    }
}

fn main() {}