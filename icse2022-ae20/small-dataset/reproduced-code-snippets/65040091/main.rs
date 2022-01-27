fn main() {
    let strA = "a";
    let result;

    {
        let strB = "abc";
        result = longest(strA, strB); // Will return strB
    }

    println!("The longest string is {}", result); // result now point to strB!!
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}