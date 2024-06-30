use std::cell::Cell;

struct Variant<'a>(&'a u32);

struct Invariant<'a>(Cell<&'a u32>);

impl<'a> Variant<'a> {
    fn foo(&'a self) -> &'a u32 {
        self.0
    }
}

impl<'a> Invariant<'a> {
    fn foo(&self) -> &u32 {
        self.0.get()
    }
}

fn main() {
    let val = 0;
    let mut variant = Variant(&val);
    let mut invariant = Invariant(Cell::new(&val));
    {
        let r = variant.foo();
    }
    variant = Variant(&val);

    {
        let r = invariant.foo();
    }
    invariant = Invariant(Cell::new(&val));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant() {
        let val = 0;
        let variant = Variant(&val);
        let r = variant.foo();
        assert_eq!(r, &val);
        let invariant = Invariant(Cell::new(&val));
        let r = invariant.foo();
        assert_eq!(r, &val);
    }
}