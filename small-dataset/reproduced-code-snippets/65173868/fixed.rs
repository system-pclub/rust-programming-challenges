/// Return a String with all characters masked as '#' except the last 4.
fn maskify(cc: &str) -> String {
    let chars = cc.chars();
    let count = chars.clone().count();
    chars
        .enumerate()
        .map(|(i, c)| {
            if i > count - 4 { '#' } else { c }
        })
        .collect()    
}

fn main() {}