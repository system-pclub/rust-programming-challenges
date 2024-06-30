use std::io::{BufReader, prelude::*};
use std::fs::File;

fn read() -> Vec<Vec<&'static str>> {
 let file = File::open("~/test").expect("failed to read file");
 let reader = BufReader::new(file);
 let mut main_vector: Vec<Vec<&str>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(v) => {
                let mut sub_vector: Vec<&str> = Vec::new();
                for element in v.split_whitespace().collect::<Vec<&str>>() {
                    sub_vector.push(element);
                }
                main_vector.push(sub_vector);
            },
            Err(e) => panic!("failed to parse: {:?}", e),
        }
    }
    return main_vector
}

fn main() {}