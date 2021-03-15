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
    let stream = self.cstream.try_clone().unwrap();
    let mut br = BufReader::new(&stream);
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

fn main() {}