#[derive(Debug)]
struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
 fn set(&mut self, r: &'a i32) {
   self.x = r;
 }
}


fn main() {
    let v = 5;
    let w = 7;
    let mut f = Foo { x: &v };

    println!("f is {:?}", f);

    f.set(&w);

    println!("now f is {:?}", f);
}
