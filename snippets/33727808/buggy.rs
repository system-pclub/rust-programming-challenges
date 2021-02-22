use std::collections::HashMap;

fn increment(map: &mut HashMap<i32, i32>, keys: &[i32]) {
    for value in keys.iter().filter_map(|index| map.get_mut(index)) {
        *value += 1;
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);
    map.insert(4,5);
    increment(&mut map, &[0, 1, 2]);
    assert!(*map.get(&1).unwrap() == 3);
    assert!(*map.get(&4).unwrap() == 5);
}
