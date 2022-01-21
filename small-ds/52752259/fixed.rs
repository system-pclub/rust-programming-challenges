fn make_fn_once<'a, T, F: FnOnce() -> T>(f: F) -> F{
    f
}


fn main() {
    let mut y = 10;
    let mut f = make_fn_once(|| {
        &mut y
    });
    f();

}
  
