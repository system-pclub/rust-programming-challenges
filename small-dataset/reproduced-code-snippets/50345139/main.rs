// no compilation error


fn get_iter() -> impl Iterator<Item = i32> {
    [1, 2, 3].iter().map(|&i| i)
}

fn main() {
    let _it = get_iter();
}
