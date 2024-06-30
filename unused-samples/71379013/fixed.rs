
use std::collections::HashMap;
fn foo() {
    let items = vec![0.2, 1.5, 0.22, 0.8, 0.7, 2.1];
    let mut groups: HashMap<u32, String> = HashMap::new();

    let mut group = |idx: f32| -> &mut String {
        let rounded = (idx / 0.2).floor() as u32;
        groups
            .entry(rounded)
            .or_insert_with(|| format!("{}:", rounded))
    };

    for item in items.iter() {
        group(*item).push_str(&format!(" {}", item))
    }
}

fn main() {}
