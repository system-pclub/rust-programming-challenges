struct Ref;

#[derive(Copy, Clone)]
struct Container<'a> {
  r : &'a Ref
}

struct ContainerB<'a> {
  c : Container<'a>
}

trait ToC {
  fn to_c<'a>(&self, r : &'a Ref) -> Container<'a>;
}

impl<'a> ToC for ContainerB<'a> {
  fn to_c(&self, r : &'a Ref) -> Container<'a> {
    self.c
  }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let r = Ref;
        let c = Container { r: &r };
        let cb = ContainerB { c };
        let ret = cb.to_c(&r);
        assert!(std::ptr::eq(ret.r, &r));
    }
}