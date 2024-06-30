fn merge(mut l1: Vec<(String, u32)>, mut l2: Vec<(String, u32)>) -> Vec<(String, u32)> {
    let mut d1 = l1.drain(..);
    let mut d2 = l2.drain(..);
    let mut result = Vec::new();
    let mut v1 = d1.next();
    let mut v2 = d2.next();
    loop {
        match (v1, v2) {
            (None, None) => return result,
            (None, Some(p2)) => {
                result.push(p2);
                v1 = None;
                v2 = d2.next()
            }
            (Some(p1), None) => {
                result.push(p1);
                v1 = d1.next();
                v2 = None
            }
            (Some(p1 @ (s1, _)), o2 @ Some((s2, _))) if s1 < s2 => {
                result.push(p1);
                v1 = d1.next();
                v2 = o2
            }
            (o1 @ Some((s1, _)), Some(p2 @ (s2, _))) if s1 > s2 => {
                result.push(p2);
                v1 = o1;
                v2 = d2.next()
            }
            (Some((s1, t1)), Some((_, t2))) => {
                result.push((s1, t1 + t2));
                v1 = d1.next();
                v2 = d2.next()
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l1 = vec![("a".to_string(), 1), ("c".to_string(), 3)];
        let l2 = vec![("b".to_string(), 2), ("c".to_string(), 4)];
        let merged = merge(l1, l2);
        assert_eq!(
            merged,
            vec![
                ("a".to_string(), 1),
                ("b".to_string(), 2),
                ("c".to_string(), 7)
            ]
        );
    }
}
