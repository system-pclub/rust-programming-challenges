struct Foo;

fn into_iterator(myvec: &Vec<Foo>) -> Box<dyn Iterator<Item = &Foo> > {
	Box::new(myvec.iter())
}

fn main() {
	
}
