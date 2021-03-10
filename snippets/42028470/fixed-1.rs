struct Foo;

fn into_iterator<'a>(myvec: &'a Vec<Foo>) -> Box<dyn Iterator<Item = &Foo> + 'a> {
	Box::new(myvec.iter())
}

fn main() {
	
}
