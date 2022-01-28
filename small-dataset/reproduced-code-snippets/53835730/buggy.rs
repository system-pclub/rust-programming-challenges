pub fn foo<T: Copy>(mut x: &T) {
    let y_owned = *x;
    let mut y = &y_owned;
    for _ in 0..10 {
        do_work(x, y);
        std::mem::swap(&mut x, &mut y);
    }
}

fn do_work<T>(_x: &T, _y: &T) {}

fn main() {}