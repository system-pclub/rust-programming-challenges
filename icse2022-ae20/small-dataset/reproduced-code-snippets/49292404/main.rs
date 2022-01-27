// no error

fn main() {
    let x = "abcd";
    let result;
    {
        let y = "qwerty";
        result = longest(x, y);
    }
    println!("The longest string is {}  ", result);

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

