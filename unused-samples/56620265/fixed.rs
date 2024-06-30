use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let sentences_path = Path::new("csv/testSentences.csv");
    let sentences = BufReader::new(File::open(&sentences_path).unwrap());

    let sentences: Vec<_> = sentences.lines().collect();
    let total = sentences.len();
    for (index, sentence) in sentences.into_iter().enumerate() {
        let line = sentence.unwrap();
        println!("Processed {} of {}, {}", index, total, line);
    }

    println!("Total {}", total);
}
