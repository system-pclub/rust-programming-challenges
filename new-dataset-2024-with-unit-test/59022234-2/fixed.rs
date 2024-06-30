fn task(chars: [u8; 3]) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    {
        let s = std::str::from_utf8(&chars).unwrap();
        v.push(s.to_string());
    }
    v
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let chars = [b'x', b'y', b'z'];
        assert_eq!(task(chars), vec!["xyz"]);
    }
}