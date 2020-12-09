use std::ptr;

enum Foo<T> {
    Bar(T),
    Baz(T),
}

impl<T> Foo<T> {
    fn switch(&mut self) {
        // I copied this code from Stack Overflow without reading
        // the surrounding text that explains why this is safe.
        unsafe {
            let tmp = ptr::read(self);
    
            // Must not panic before we get to `ptr::write`

            let new = match tmp {
                Foo::Bar(val) => Foo::Baz(val),
                Foo::Baz(val) => Foo::Bar(val),
            };
    
            ptr::write(self, new);
        }
    }
}

fn main() {}