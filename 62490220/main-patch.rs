#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::string::String;

struct Args {
    path: String,
    pattern: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args {
        path: "path".to_string(),
        pattern: "pattern".to_string()
    };
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {  // line 1
        let line = line?;
        if line.contains(&args.pattern) { //line 2
            println!("{}", line);  
        }
    }

    Ok(())
}
