trait Animal {
  fn make_sound(&self) -> &'static str;
}

struct Dog {}
impl Animal for Dog{
  fn make_sound(&self) -> &'static str {
    "woof"
  }
}

struct Cat{}
impl Animal for Cat{
  fn make_sound(&self) -> &'static str {
    "meow"
  }
}

struct Person {
  my_dog: Dog,
  my_cat: Cat
}

impl Person {
  fn look_up_animal(&self, animal_name: &str) -> Box<&dyn Animal> {
    match animal_name {
      "dog" => Box::from(&self.my_dog as &dyn Animal),
      "cat" => Box::from(&self.my_cat as &dyn Animal),
      &_ => Box::from(&self.my_dog as &dyn Animal)
    }
  }
}
fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_person() {
    let person = Person{my_dog: Dog{}, my_cat: Cat{}};
    let dog = person.look_up_animal("dog");
    assert_eq!(dog.make_sound(), "woof");
    let cat = person.look_up_animal("cat");
    assert_eq!(cat.make_sound(), "meow");
  }
}