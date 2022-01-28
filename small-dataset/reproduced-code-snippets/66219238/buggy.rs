struct Person {
    name: String,
}

fn main() {
    // Create a mutable vector
    let mut people: Vec<Person> = ["Joe", "Shavawn", "Katie"]
        .iter()
        .map(|&s| Person {
            name: s.to_string(),
        })
        .collect();

    // Borrow a reference to an element
    let person_ref = &people[0];

    // Mutate the vector
    let new_person = Person {
        name: "Tim".to_string(),
    };
    people.push(new_person);

    // Attempt to use the borrowed reference
    assert!(person_ref.name == "Joe");
}