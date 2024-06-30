use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

fn main() {
    let sentences_path = Path::new("csv/testSentences.csv");
    let sentences = BufReader::new(File::open(&sentences_path).unwrap());

    let mut lines = sentences.lines();
    let total = lines.by_ref().count();

    for (index, sentence) in sentences.lines().enumerate() {
        let line = sentence.unwrap();
        println!("Processed {} of {}, {}", index, total, line);
    }

    println!("Total {}", total);
}