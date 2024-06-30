fn task(input: i32) -> String {
    let nums = vec![input, input + 1, input + 2];

    let takes_nums = || nums;
    format!("{:?} {:?}", takes_nums(), nums)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        assert_eq!(task(42), "[42, 43, 44] [42, 43, 44]");
    }
}