fn func_that_returns<T>(value: T) -> impl FnOnce() -> T {
    move || value
}

fn main() {
    let f = func_that_returns(3);
    assert_eq!(f(), 3);
}
