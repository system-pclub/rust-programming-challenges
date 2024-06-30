struct Person {
    name: String,
    age: u8,
}

static NAME: for<'a> fn(&'a Person) -> &'a String = |p: &Person| &p.name;
static AGE: fn(&Person) -> u8 = |p: &Person| p.age;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let p = Person {
            name: "Nobody".to_string(),
            age: 24,
        };
        let name = NAME(&p);
        let age = AGE(&p);
        assert_eq!(name, "Nobody");
        assert_eq!(age, 24);
    }
}