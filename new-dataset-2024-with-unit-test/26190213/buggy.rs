use std::thread::spawn;

#[derive(Debug)]
struct Node<'a> {
    left: &'a i32,
    right: &'a i32,
}

fn task(x: i32, y: i32) -> String {
    let l = x;
    let r = y;
    let n = Node {
        left: &l,
        right: &r,
    };

    let handler = spawn(|| {
        format!("{:?}", n)
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