
struct Haha {
    pub a: u32,
    pub b: Vec<u32>,
}

fn main() {
  let example = Haha {
      a: 32,
      b: vec![1],
  }; 
  let _new_a = example.a;
  let _new_b = &example.b;

  println!("{:?}", example.b);
}
