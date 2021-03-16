#[derive(Clone)]
struct A(u32);
#[derive(Clone)]
struct B<'a>(u32, &'a u32);

impl A {
    fn next(&self) -> A {
        let mut new = self.clone();
        new.0 = new.0 + 1;
        new
    }
}

impl<'a> B<'a> {
    fn next(&self) -> B {
        let mut new = self.clone();
        new.0 = new.0 + 1;
        new
    }
}

fn main() {
    let mut a = A(0);
    for _ in 0..5 {
        a = a.next();
    }
    
    let x = 0;
    let mut b = B(0, &x);
    for _ in 0..5 {
        b = b.next();
    }
}