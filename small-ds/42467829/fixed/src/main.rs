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
    let ref_urls = &urls;
    rayon::scope(move |s| {
        s.spawn(move |_| {
            for _ in ref_urls {
                println!("{}", rx.recv().unwrap());
            }
        });
        for url in ref_urls {
            let tx = tx.clone();
            s.spawn(move |_| {
                let connector = HttpsConnector::new(url);
                let mut result = String::new();
                tx.send(result).unwrap();  
            });
        }
    });
}

fn main() {}