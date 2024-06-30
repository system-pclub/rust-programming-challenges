struct MyBorrows<'a> {
    val : &'a i32
}

impl <'a> MyBorrows<'a> {
    fn new(v : &'a i32) -> MyBorrows<'a> {
        MyBorrows { val : v }
    }
}

fn task(my_val: i32) -> String {
    let mut my_vec = vec![];
    my_vec.push(Box::new(MyBorrows::new(&my_val)));
    for _ in 1..4 {
        let last = my_vec.last().unwrap();
        let new_borrow = Box::new(MyBorrows::new(last.val));
        my_vec.push(new_borrow);
    }
    let ret = my_vec.iter().map(|x| x.val.to_string()).collect::<Vec<String>>().join(" ");
    ret
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_borrows() {
        let my_val = 23;
        let my_vec = task(my_val);
        assert_eq!(my_vec, "23 23 23 23");
    }
}