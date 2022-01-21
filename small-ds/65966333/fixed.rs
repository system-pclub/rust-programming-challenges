fn main() {
    let c = [4,5,6,1];
    let min = argmin(&c);
    println!("{}", min);
}

fn argmin(arr: &[i32]) -> usize {
        arr.iter()
            .enumerate()
            .min_by_key(|&(_, v)| v)
            .map(|(idx, _)| idx)
            .unwrap()
}