fn task(chars: [u8; 3]) -> Vec<&str> {
    let mut v: Vec<&str> = Vec::new();
    {
        let chars = &chars.clone();
        let s = std::str::from_utf8(chars).unwrap();
        v.push(s);
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