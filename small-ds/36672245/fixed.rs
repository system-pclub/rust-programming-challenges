fn merge(mut l1: Vec<(String, u32)>, mut l2: Vec<(String, u32)>) -> Vec<(String, u32)> {
    let mut d1 = l1.drain(..);
    let mut d2 = l2.drain(..);
    let mut result = Vec::new();
    let mut v1 = d1.next();
    let mut v2 = d2.next();
    loop {
        match (&v1, &v2) {
            (None, None) => return result,
            (None, Some(x)) => {
                result.push(x.clone());
                v2 = d2.next()
            }
            (Some(x), None) => {
                result.push(x.clone());
                v1 = d1.next()
            }
            (Some(p1), Some(p2)) => {
                let (ref s1, t1) = p1;
                let (ref s2, t2) = p2;
                if s1 == s2 {
                    result.push((s1.clone(), t1 + t2));
                    v1 = d1.next();
                    v2 = d2.next();
                } else if s1 < s2 {
                    result.push(p1.clone());
                    v1 = d1.next();
                } else {
                    result.push(p2.clone());
                    v2 = d2.next();
                }
            }
        }
    }
}

fn main() {}
