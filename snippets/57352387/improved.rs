use std::marker::PhantomData;

/// Example struct.
struct Obj<'a>(PhantomData<&'a ()>);

/// A struct that lets you build new `Obj`s
struct ObjFactory;

impl ObjFactory {
    /// makes a new object
    fn make(&self) -> Obj<'_> {
        Obj(PhantomData) // the PhantomData has the same lifetime as `self`
    }

    /// enforce that there are no objects from this ObjFactory
    fn recall(&mut self) {}
}

fn main() {
    // make factory
    let mut obj_factory = ObjFactory;

    // create an object
    let obj = obj_factory.make();

    // uh oh! no objects are allowed!
    obj_factory.recall();

    // obj lasts until the end of the function
    core::mem::drop(obj);
}