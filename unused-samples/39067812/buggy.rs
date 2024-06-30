use std::io::BufReader;
use std::io::BufRead;
use std::net::{TcpStream};
use std::io::Write;

#[derive(Debug)]
pub struct Paradise {
  cstream: TcpStream
}

impl Paradise {
  pub fn new(stream: TcpStream) -> Paradise {
    Paradise {cstream: stream}
  }

  pub fn start(&mut self) {
    self.write_message(220, "Welcome to Paradise");
    let mut br = BufReader::new(&self.cstream);
    loop {
      let mut buffer = String::new();
      let _ = br.read_line(&mut buffer);
      println!("{:?}", buffer);
      self.write_message(550, "No");
    }
  }

  pub fn write_message(&mut self, code: i32, message: &str) {
    let foo = format!("{} {}\r\n", code, message);
    let _ = self.cstream.write(foo.as_bytes());
  }
}

use std::net::TcpListener;
use std::thread;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:2121").unwrap();
  println!("listening started, ready to accept");
  for stream in listener.incoming() {
    match stream {
      Err(e) => { println!("failed: {}", e) }
      Ok(stream) => {
        let addr = stream.peer_addr().unwrap();
        println!("Got connection from {}", addr);

        let mut p = Paradise::new(stream);
        println!("{:?}", p);
        thread::spawn(move || {
          p.start();
        });
      }
    }
  }
}