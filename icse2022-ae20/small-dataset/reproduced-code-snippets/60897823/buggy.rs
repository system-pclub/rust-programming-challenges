struct Person {
    name : String,
    age : u8,
}
 
 
fn main() {
    let p = Person{ name: "Nobody".to_string(), age : 24};
 
    let age = |p : &Person| p.age;
    let name = |p : &Person | &p.name;
 
    println! ("name={}, age={}" , name(&p), age(&p));
}
