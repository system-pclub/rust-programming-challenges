use std::collections::HashMap;

fn increment(map: &mut HashMap<i32, i32>, keys: &[i32]) {
    let _xs: Vec<_> = keys.iter().filter_map(|index| map.get_mut(index)).collect();
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(4, 5);
        increment(&mut map, &[0, 1, 2]);
        assert!(*map.get(&1).unwrap() == 3);
        assert!(*map.get(&4).unwrap() == 5);
    }
}
