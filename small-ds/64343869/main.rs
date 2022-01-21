// no compilation error

fn main() {
    let x;
    {
        let y = &13;
        x = y;
    }
    println!("{}", x); 
}
