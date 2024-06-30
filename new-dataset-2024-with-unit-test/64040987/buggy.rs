fn test_vec() -> String {
    let src = vec![10u8, 20u8];
    let a = &src[0];
    drop(src); // rejected by compiler
    let b = a;
    format!("{:?}", b)
}

fn test_array() -> String {
    let src = [10u8, 20u8];
    let a = &src[0];
    drop(src);
    let b = a;
    format!("{:?}", b)
}

fn test_box() -> String {
    let src = Box::new(10u8);
    let a = &src;
    let b = a;
    format!("{:?}", b)
}

fn test_var() -> String {
    let src = 10u8;
    let a = &src;
    drop(src);
    let b = a;
    format!("{:?}", b)
}

fn main() {}
#[cfg(test)]
mod tests {

    #[test]
    fn test_vec() {
        assert_eq!(super::test_vec(), "10");
    }

    #[test]
    fn test_array() {
        assert_eq!(super::test_array(), "10");
    }

    #[test]
    fn test_box() {
        assert_eq!(super::test_box(), "10");
    }

    #[test]
    fn test_var() {
        assert_eq!(super::test_var(), "10");
    }
}