fn modify_vec_get_slice(values: &mut Vec<i32>) -> &[i32] {
    // Modify "values" in some way
    values.push(5);

    // Return a slice into "values" (dummy code)
    values.split_at(values.len() / 2).1
}

fn use_slice(slice: &[i32]) -> String {
    // Do something immutably with the slice
    slice
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn use_vec(values: &Vec<i32>) -> String {
    // Do something immutably with the vec
    values
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn task(values: Vec<i32>) -> String {
    let mut values = values;
    let mut ret = String::new();
    {
        let slice = modify_vec_get_slice(&mut values); // use values mutably
        ret = use_slice(slice); // use slice immutably (which refers to the vec?). Uncommenting this line fixes the error

    }
    let ret2 = use_vec(&values);

    format!("{} {}", ret2, ret)
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let values = vec![1, 2, 3, 4];
        let ret = task(values);
        assert_eq!(ret, "1 2 3 4 5 3 4 5")
    }
}
