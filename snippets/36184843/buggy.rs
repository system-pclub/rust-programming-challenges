use std::fmt;
use std::error::{self, Error};

#[derive(Debug)]
pub enum MyError {
    NotFound(String),
    PersistenceError(Box<Error + Send + Sync>),
}

pub type MyResult<T> = Result<T, MyError>;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::NotFound(ref msg) => write!(f, "Not found: {}", msg),
            MyError::PersistenceError(ref cause) => write!(f, "Persistence error: {}", cause),
        }
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        match *self {
            MyError::NotFound(ref msg) => msg,
            MyError::PersistenceError(ref cause) => cause.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            MyError::NotFound(_) => None,
            // `*cause` does not live long enough
            MyError::PersistenceError(cause) => Some(&*cause),
        }
    }
}

fn main() {}