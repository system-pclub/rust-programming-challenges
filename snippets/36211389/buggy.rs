use std::sync::mpsc;
use std::thread;

const THREAD_COUNT: u64 = 4;

fn average<F: Fn(f64) -> f64 + Send + 'static>(f: F) -> f64 {
    let (tx, rx) = mpsc::channel();
    for id in 0..THREAD_COUNT {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(f(id as f64));
        });
    }

    let mut total = 0.0;
    for id in 0..THREAD_COUNT {
        total += rx.recv().unwrap();
    }
    total / THREAD_COUNT as f64
}

fn main() {
    average(|x: f64| -> f64 { x });
}