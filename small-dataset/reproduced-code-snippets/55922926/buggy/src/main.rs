extern crate chrono; // 0.4.6

fn main() {
    let mut now = chrono::Local::today();
    now = std::mem::replace(&mut now, now.succ());
}