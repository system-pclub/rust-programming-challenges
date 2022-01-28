/// new placeholder struct that should take up no space
struct Dummy;

/// Example struct.
/// Now has a dummy reference so the compiler knows when to get mad
struct Obj<'a>(&'a Dummy);

/// A struct that lets you build new `Obj`s
struct ObjFactory {dummy: Dummy}

impl ObjFactory {

    /// makes a new object
    fn make(&self) -> Obj {
        // let the `Obj` immutably borrow the dummy
        Obj(&self.dummy)
    }

    /// enforce that there are no objects from this ObjFactory
    fn recall(&mut self) {
        // mutably borrow `dummy`, which means that Obj's can't borrow it anymore.
        let _borrow = &mut self.dummy;
    }
}

fn main() {

    // make factory
    let mut obj_factory = ObjFactory{dummy: Dummy};

    // create an object
    let obj = obj_factory.make();

    // uh oh! no objects are allowed!
    obj_factory.recall();

    // obj lasts until the end of the function
    core::mem::drop(obj);

}