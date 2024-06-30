pub struct Rule<'a> {
    selector: &'a str,
}

impl<'a> Rule<'a> {
    pub fn new(selector: &'a str) -> Self {
        Self { selector }
    }

    pub fn get_selector(&self) -> &'a str {
        self.selector
    }

    pub fn set_selector(&mut self, selector: &'a str) {
        self.selector = selector
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule() {
        let mut a_rule = Rule::new(".foo");
        a_rule.set_selector(".bar");
        assert_eq!(a_rule.get_selector(), ".bar");
    }
}