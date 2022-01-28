trait MyTrait<'a> {
    type N: 'a;

    fn func(&'a self) -> Self::N;
}

struct MyPtr<'a> {
    x: &'a usize,
}

struct MyStruct {
    data: Vec<usize>,
}

impl<'a> MyTrait<'a> for MyStruct {
    type N = MyPtr<'a>;

    fn func(&'a self) -> Self::N {
        MyPtr { x: &self.data[0] }
    }
}

fn myfunc<'a, T: 'a + MyTrait<'a, N = MyPtr<'a>>>(g: T) {
    println!("{}", g.func().x);
}

fn main() {
    let y = MyStruct {
        data: (15..20).collect(),
    };
    myfunc(y);
}
