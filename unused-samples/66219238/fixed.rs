use std::rc::Rc;

struct Person {
    name: String,
}

fn main() {
    // Create a mutable vector
    let mut people: Vec<Rc<Person>> = ["Joe", "Shavawn", "Katie"]
        .iter()
        .map(|&s| {
            Rc::new(Person {
                name: s.to_string(),
            })
        })
        .collect();

    // Borrow a reference to an element
    let person_ref = Rc::clone(&people[0]);

    // Mutate the vector
    let new_person = Rc::new(Person {
        name: "Tim".to_string(),
    });
    people.push(new_person);

    // Attempt to use the borrowed reference
    assert!(person_ref.name == "Joe");
}