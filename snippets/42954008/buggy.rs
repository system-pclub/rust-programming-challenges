fn main() {
    let my_array = vec![61, 14, 71, 23, 42, 8, 13, 66];
    let max = max(my_array);
    let _min = min(my_array);
    println!("Max value is {}.", max);
}

fn max(array: Vec<i32>) -> i32 {
    let mut max = array[0];
    for val in array {
        if max < val {
            max = val;
        }
    }
    max
}

fn min(array: Vec<i32>) -> i32 {
    let mut min = array[0];
    for val in array {
        if min > val {
            min = val;
        }
    }
    min
}
