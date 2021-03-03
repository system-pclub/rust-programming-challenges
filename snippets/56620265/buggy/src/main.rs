extern crate regex;

use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

fn main() {
    let sentences_path = Path::new("csv/testSentences.csv");
    let sentences = BufReader::new(File::open(&sentences_path).unwrap());

    let total = sentences.lines().count();

    for (index, sentence) in sentences.lines().enumerate() {
        let line = sentence.unwrap();
        println!("Processed {} of {}, {}", index, total, line);
    }

    println!("Total {}", total);
}