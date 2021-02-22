use std::fmt::Debug;
use std::sync::Arc;
use std::thread;

struct Wrapper<T: Debug + Sync + Send + 'static> {
    arc_val: Arc<T>,
}

fn run_thread<T: Debug + Sync + Send + 'static>(wrapper: &Wrapper<T>) {
    let arc_val = wrapper.arc_val.clone();
    let thr = thread::spawn(move || {
        println!("{:?}", *arc_val);
    });

    thr.join().unwrap();
}

fn main() {
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);
    }

    let w = Wrapper { arc_val: Arc::new(v) };
    run_thread(&w);

    println!("{}", (*w.arc_val)[0]);
}
