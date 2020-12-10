// no error

struct Value<'v>(&'v ());
struct Container {}

impl Container {
    fn get<'v>(&'v self) -> Value<'v> {
        todo!()
    }
    
    fn set<'v>(&'v self, x: Value<'v>) {
        todo!()
    }
}

fn main() {}