fn main() {
    let s = String::from("Hello!");
    let parse_this = parse_string(&s);
    println!("{}", parse_this("goodbye!"));
}

fn parse_string<'a>(string: &'a String) -> impl Fn(&str) -> &'a String {
    return move |target_string| {
        // pretend there is parsing logic
        println!("{}", target_string);
        string
    };
}