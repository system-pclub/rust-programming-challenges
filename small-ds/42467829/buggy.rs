use std::sync::mpsc;
use std::thread;

struct HttpsConnector;
impl HttpsConnector {
    fn new(s: &str) -> Self {
        HttpsConnector {}
    }
}

fn fetch(urls: Vec<&str>) {
    let (tx, rx) = mpsc::channel();

    for url in urls {
        let tx = tx.clone();

        thread::spawn(move || {
            let connector = HttpsConnector::new(url);
            let mut result = String::new();
            tx.send(result).unwrap();  
        });
    }

    //let mut result: Vec<String> = vec![];
    for _ in urls {
        println!("{}", rx.recv().unwrap());
    }
}

fn main() {}