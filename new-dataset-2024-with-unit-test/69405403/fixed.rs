trait Dynamic {
    fn foo(&self) -> String;
}

struct Concrete {
    bar: i32,
}

impl Concrete {
    fn new(value: i32) -> Self {
        Self { bar: value }
    }
}

impl Dynamic for Concrete {
    fn foo(&self) -> String {
        format!("Concrete: {}", self.bar)
    }
}

fn generate<'a>() -> Vec<Box<dyn Dynamic + 'a>> {
    let temp: Vec<Box<dyn Dynamic>> = vec![Box::new(Concrete::new(1)), Box::new(Concrete::new(3))];
    temp
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let result = generate();
        assert_eq!(result[0].foo(), "Concrete: 1");
        assert_eq!(result[1].foo(), "Concrete: 3");
    }
}