fn main() {
    let names = vec!["foo", "bar", "baz"];
    let print = printer(names);
    let result = print();
    println!("{}", result);
    do_other_thing(names.as_slice());
}

fn printer(names: Vec<&str>) -> Box<Fn() -> String> {
    Box::new(move || {
        let text = String::new();
        for name in names {
            text = text + name;
        }
        text
    })
}

fn do_other_thing(names: &[&str]) {}