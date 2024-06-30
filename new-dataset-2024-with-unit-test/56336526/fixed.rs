use std::thread;

#[derive(Debug, Clone)]
struct Test {
    test_string: String,
}

trait Example {
    fn tst(&self) -> String;
}

impl Example for Test {
    fn tst(&self) -> String {
        format!("Hello, {}", self.test_string)
    }
}

fn run_concrete_test(tester: &Test) -> String {
    let t = tester.clone();
    let handler = thread::spawn(move || t.tst());
    handler.join().unwrap()
}

fn run_trait_test<F>(tester: &F) -> String
where
    F: Example + Sync + Send + Clone + 'static,
{
    let t = tester.clone();
    let handler = thread::spawn(move || t.tst());
    handler.join().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_concrete_test() {
        let x = Test {
            test_string: "test string".to_string(),
        };
        assert_eq!(run_concrete_test(&x), "Hello, test string");
    }

    #[test]
    fn test_run_trait_test() {
        let x = Test {
            test_string: "test string".to_string(),
        };
        assert_eq!(run_trait_test(&x), "Hello, test string");
    }
}