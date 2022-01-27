fn main() {
    let num = 5;
    let nums = vec![1, 2, 3];
    let plus_num = |x: i32| x + num;
    let takes_nums = || nums;
    assert_eq!(10, plus_num(5));
    println!("{:?}", nums);
}