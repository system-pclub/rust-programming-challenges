fn main() {
    let parse_this = parse_string(&String::from("Hello!"));
    println!("{}", parse_this("goodbye!"));
}

fn parse_string(string: &String) -> impl FnOnce(&str) -> &String {
    return |targetString| {
        // pretend there is parsing logic
        println!("{}", targetString);
        return string;
    };
}