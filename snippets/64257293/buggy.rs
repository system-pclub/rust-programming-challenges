pub struct ParseError {}

pub type Parser<T> = Box<dyn Fn(&str) -> Result<(&str, T), ParseError>>;

pub fn pure<T: 'static>(value: T) -> Parser<T> {
    Box::new(move |input: &str| -> Result<(&str, T), ParseError> { Ok((input, value)) })
}

fn main() {}