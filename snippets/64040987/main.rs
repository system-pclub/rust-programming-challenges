// No fix. The asker made the example asking for explanation,
// not for fix


fn foo() {
    let vector = vec![1u8, 2u8];
    let a = &vector.as_slice()[0];
    drop(vector);
    let _b = a;
}

fn bar() {
    let array = [1u8, 2u8];
    let a = &array[0];
    drop(array);
    let _b = a;
}

fn main() {}
