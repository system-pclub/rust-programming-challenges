use std::collections::HashMap;

fn increment(map: &mut HashMap<i32, i32>, keys: &[i32]) {
    for index in keys {
        if let Some(value) = map.get_mut(index) {
            *value += 1;
        }
    }
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
