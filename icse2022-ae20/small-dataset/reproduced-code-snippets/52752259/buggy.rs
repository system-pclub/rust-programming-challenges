// no fix.

fn main() {
    let mut y = 10;
    let mut f = || &mut y;
    f();
}
  