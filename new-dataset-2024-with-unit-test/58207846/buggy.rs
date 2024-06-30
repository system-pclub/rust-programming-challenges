fn parse_string(string: &String) -> impl Fn(&str) -> (&String, String) {
    return move |target_string| {
        let target_string = target_string.replace(string, " ");
        (&string, target_string)
    };
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let s = String::from("Hello!");
        let parse_this = parse_string(&s);
        let (string, target_string) = parse_this("goodbye!");
        assert_eq!(string, "Hello!");
        assert_eq!(target_string, "goodbye!");
    }
}