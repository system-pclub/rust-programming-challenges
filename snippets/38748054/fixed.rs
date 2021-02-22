fn main() {
    let names = vec!["foo", "bar", "baz"];
    let print = printer(&names);
    let result = print();
    println!("{}", result);
    do_other_thing(names.as_slice());
}

fn printer<'a>(names: &'a Vec<&str>) -> Box<Fn() -> String + 'a>{
    Box::new(move || {
        let mut text = String::new();
        for name in names {
            text = text + name;
        }
        text
    })
}

fn do_other_thing(names: &[&str]) {}