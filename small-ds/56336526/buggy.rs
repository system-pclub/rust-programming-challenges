use std::thread;

#[derive(Debug, Clone)]
struct Test {
    test_string: String,
}

trait Example {
    fn tst(&self) -> ();
}

impl Example for Test {
    fn tst(&self) {
        println!("{:?}", self);
    }
}

// compiles, no 'static here
fn run_concrete_test(tester: &Test) {
    let t = tester.clone();
    thread::spawn(move || {
        t.tst();
    });
}

// compiles with 'static
// but F shouldn't be only static
fn run_trait_test<F>(tester: &F)
where
    F: Example + Sync + Send + 'static,
{
    let t = tester.clone();
    let store_t = thread::spawn(move || {
        t.tst();
    });
}

fn main() {
    //does run, no static
    let x = Test {
        test_string: "test string".to_string(),
    };
    run_concrete_test(&x);

    // doe sn't compile because of static
    run_trait_test(&x);
    println!("{:?}", x);
}