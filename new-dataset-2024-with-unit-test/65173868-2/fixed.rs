/// Return a String with all characters masked as '#' except the last 4.
fn maskify(cc: &str) -> String {
    let chars = cc.chars();
    chars
        .enumerate()
        .map(|(i, c)| if i < cc.len() - 4 { '#' } else { c })
        .collect()
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maskify() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("64607935616"), "#######5616");
    }
}
