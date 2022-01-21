use std::fmt::Debug;
use std::thread;

struct Wrapper<T: Debug + Send + 'static> {
    val: T,
}

fn run_thread<T: Debug + Send + 'static>(wrapper: Wrapper<T>) {
    let thr = thread::spawn(move || {
        println!("{:?}", wrapper.val);
    });

    thr.join();
}

fn main() {
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);
    }

    run_thread(Wrapper { val: &v });
}
