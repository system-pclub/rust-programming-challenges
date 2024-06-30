type Foo = Vec<u8>;

fn quux(_: Foo) -> Option<Foo> {
    Some(Vec::new())
}

  
// Now consider a type that is somehow conceptually very similar to `Box`:

struct NotBox<T> {
    contents: T
}

// We can write a function that temporarily moves out contents of the `NotBox` and puts something back in before returning it:



// I want to write an analogous function that works with `Box`es but the compiler does not like it:

    fn baz(mut abox: Box<Foo>) -> Option<Box<Foo>> {
        let foo = *abox; // now `abox` is "empty"
        match quux(foo) {
            Some(new_foo) => {
                *abox = new_foo; // error: use of moved value: `abox`
                Some(abox)
            }
            None => None
        }
    }

// I could return `Some(Box::new(new_foo))` instead but that performs unnecessary allocation - I already have some memory at my disposal! Is it possible to avoid that?

// I would also like to get rid of the `match` statements but again the compiler is not happy with it (even for the `NotBox` version):

    fn bar(mut notbox: NotBox<Foo>) -> Option<NotBox<Foo>> {
        let foo = notbox.contents;
        quux(foo).map(|new_foo| {
            notbox.contents = new_foo; // error: capture of partially moved value: `notbox`
            notbox
        })
    }

fn main() {}


// ---

// Moving out of boxes is special-cased in the compiler. You can move something out of them, but you can't move something back in, because the act of moving out also deallocates. You can do something silly with `std::ptr::write`, `std::ptr::read`, and `std::ptr::replace`, but it's hard to get it right, because *something* valid should be inside a `Box` when it is dropped. I would recommend just accepting the allocation, or switching to a `Box<Option<Foo>>` instead.

// ---

// So, moving out of a `Box` is a special case... now what?

// The `std::mem` module presents a number of safe functions to move values around, without poking holes (!) into the memory safety of Rust. Of interest here are `swap` and [`replace`](https://doc.rust-lang.org/std/mem/fn.replace.html):

// >     pub fn replace<T>(dest: &mut T, src: T) -> T

// Which we can use like so:

//     fn baz(mut abox: Box<Foo>) -> Option<Box<Foo>> {
//         let foo = std::mem::replace(&mut *abox, Foo::default());

//         match quux(foo) {
//             Some(new_foo) => {
//                 *abox = new_foo;
//                 Some(abox)
//             }
//             None => None
//         }
//     }

// It also helps in the `map` case, because it does not borrow the `Box`:

//     fn baz(mut abox: Box<Foo>) -> Option<Box<Foo>> {
//         let foo = std::mem::replace(&mut *abox, Foo::default());
    
//         quux(foo).map(|new_foo| { *abox = new_foo; abox })
//     }

// ---

// > We can write a function that temporarily moves out contents of the NotBox and puts something back in before returning it

// That's because you can partially move out from the struct that you take by value. It behaves as if all fields were separate variables. That is not possible though if the struct implements `Drop`, because `drop` needs the whole struct to be valid, always (in case of panic).

// As for providing workaround, you haven't provided enough information – especially, why `baz` needs to take `Box` as an argument and why `quux` can't? Which functions are yours and which are part of an API you can't change? What is the real type of `Foo`? Is it big?

// The best workaround would be not to use `Box` at all.