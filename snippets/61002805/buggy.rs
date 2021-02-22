fn main() {
    let foo = vec![('a', vec![1,2,3]), ('v', vec![2,3,4])];
    let baz: Vec<(char, i32)> = foo
        .into_iter()
        .map(|a| a.1.into_iter().map(|b| (a.0, b)))
        .flatten()
        .collect();
    println!("{:?}",baz);
}