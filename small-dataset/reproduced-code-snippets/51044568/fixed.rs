fn main() {
    println!("{:?}", f(vec![1]))
}

fn f(v: Vec<isize>) -> (Vec<isize>, isize) {
    match v.get(0) {
        Some(&a) => (v, a),
        _ => (v, 0)
    }
}