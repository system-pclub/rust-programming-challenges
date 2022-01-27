use std::io::{self, BufReader, prelude::*};
use std::fs::File;

fn read() -> Vec<Vec<String>> {
 let file = File::open("~/test").expect("failed to read file");
 let reader = BufReader::new(file);
 let mut main_vector: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(v) => {
                let mut sub_vector: Vec<String> = Vec::new();
                for element in v.split_whitespace() {
                    sub_vector.push(element.to_string());
                }
                main_vector.push(sub_vector);
            },
            Err(e) => panic!("failed to parse: {:?}", e),
        }
    }
    main_vector
}

fn main() {}