use std::collections::HashMap;

fn get_sizes<'a>(
    dir: &'a str,
    _listings: &HashMap<&str, Vec<Vec<&str>>>,
    sizes: &mut HashMap<&'a str, i32>,
) {
    sizes.insert(dir, 0);
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sizes() {
        let mut sizes = HashMap::new();
        let listings = HashMap::new();
        get_sizes("a", &listings, &mut sizes);
        assert_eq!(sizes.get("a"), Some(&0));
    }
}
