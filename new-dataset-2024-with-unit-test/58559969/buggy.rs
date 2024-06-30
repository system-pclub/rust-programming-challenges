fn accept(mut input: &Vec<String>) -> String {
    let vec = if input.len() == 0 {
        &vec!["empty".to_string()]
    } else {
        input
    };
    // ... do something with `vec`, like looping over it
    return vec[0].clone();
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accept() {
        let input = vec!["Hello".to_string(), "World".to_string()];
        assert_eq!(accept(&input), "Hello");
    }

    #[test]
    fn test_accept_empty() {
        let input = vec![];
        assert_eq!(accept(&input), "empty");
    }
}