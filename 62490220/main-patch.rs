#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::string::String;


struct MyError {
    str: String,
}

impl From<std::io::Error> for MyError {
    fn from(err: Error) -> Self {
        MyError { str: err.to_string() }
    }
}

fn open(args: &str) -> Result<i32, MyError> {
    let file = File::open(args)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {  // line 1
        let line1 = line?;  //line 3
        if line1.contains(&args) {
            println!("{}", line1);
        }
    }

    Ok(0)
}


fn main() {
    let a = "path";
    let res = open(a);
}
