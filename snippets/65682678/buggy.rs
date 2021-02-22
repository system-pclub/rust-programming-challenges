fn main() {
    let mut count = 0;
 
    let mut inc = || {
        count += 2;
    };
 
    for _index in 1..5 {
        inc();
        println!("{}", count);
    }
}
