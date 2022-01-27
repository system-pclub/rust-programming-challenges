extern crate chrono; // 0.4.6

pub fn replace<T>(src: T, dest: &mut T) -> T {
    std::mem::replace(dest, src)
}

fn main() {
    let mut now = chrono::Local::today();
    now = replace(now.succ(), &mut now);
}

