/// Return a String with all characters masked as '#' except the last 4.
fn maskify(cc: &str) -> String {
    let chars = cc.to_string().chars();
    chars
        .enumerate()
        .map(|(i, c)| {
            if i > chars.count() - 4 { '#' } else { c }
        })
        .collect()    
}

fn main() {}