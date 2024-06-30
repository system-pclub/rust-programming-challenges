pub struct Writer<'a> {
    target: &'a mut String,
}

impl<'a> Writer<'a> {
    fn indent(&mut self) -> &String {
        self.target
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer() {
        let mut target = String::new();
        {
            let mut writer = Writer { target: &mut target };
            let indent = writer.indent();
            assert_eq!(indent, "");
        }
    }
}