use std::sync::Arc;
use std::thread::spawn;

#[derive(Debug)]
struct Node {
    left: i32,
    right: i32,
}

fn task(x: i32, y: i32) -> String {
    let l = x;
    let r = y;
    let n = Arc::new(Node { left: l, right: r });

    let n_clone = n.clone();
    let handler = spawn(move || {
        format!("{:?}", n_clone)
    });
    handler.join().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let task_ret = task(3, 4);
        assert_eq!(task_ret, "Node { left: 3, right: 4 }");
    }
}
