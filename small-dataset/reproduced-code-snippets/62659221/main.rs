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

fn convert<'v1, 'v2>(x: &'v1 Container, env: &'v2 Container) {
    let root: Value<'v2> = env.get();
    x.set(root);
}

fn main() {}