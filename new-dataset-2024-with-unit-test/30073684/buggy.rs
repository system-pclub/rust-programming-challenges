fn change(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;
    *b = c;
}

fn task(input: Vec<i32>) -> Vec<i32> {
    let mut v = input;
    change(&mut v[0], &mut v[1]);
    v
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        assert_eq!(task(vec![1, 2]), vec![2, 1]);
    }
}