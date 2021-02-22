fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.into_iter().fold(0, |a, b| a + b);
    v = vec![1, 2, 3, 4];
    vec![1, 2, 3].into_iter().for_each(|x| {
        v.into_iter().fold(x, |a, b| a + b);
        v = vec![1, 2, 3, 4];
    });
}