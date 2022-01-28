// Use "rental" here

#[macro_use]
extern crate rental;

#[derive(Debug)]
pub struct Thing {
    count: u32,
}

pub struct ThingRef<'a> {
    thing_ref: &'a Thing
}

impl<'a> ThingRef<'a> {
    fn new(r: &'a Thing) -> Self {
        ThingRef {
            thing_ref: r
        }
    }
}


rental!{
    pub mod my_rental {
        use super::*;
        #[rental]
        pub struct Combined{
            thing: Box<Thing>, 
            thing_ref: Box<ThingRef<'thing>>
        }
    }

}


fn make_combined() -> my_rental::Combined {
    let thing = Box::new(Thing {count: 0});
    my_rental::Combined::new(thing, |t| Box::new(ThingRef::new(t)) )
}

fn main() {
    let combined = make_combined();
    println!("{:?}", combined.rent(|s| println!("{:?}", s.thing_ref)));
}
