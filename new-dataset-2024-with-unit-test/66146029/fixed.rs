use std::collections::HashSet;
use std::iter::FromIterator;

fn task(lines: Vec<String>) -> Vec<HashSet<u8>> {
    let mut candidates = HashSet::<&u8>::new();
    let mut sum = 0;
    let mut ret = Vec::<HashSet<u8>>::new();
    let mut _l2: String = "".to_string();
    for l in lines.iter() {
        // lines is Vec<String>
        if candidates.len() == 0 {
            _l2 = l.to_string();
            candidates = HashSet::<&u8>::from_iter(_l2.as_bytes());
            // println!("candidates = {:?}", candidates);
            ret.push(candidates.iter().map(|&x|*x).collect());
        } else if l.len() == 0 {
            sum = sum + candidates.len();
            candidates = HashSet::<&u8>::new();
        } else {
            let h2 = HashSet::<&u8>::from_iter(l.as_bytes());
            candidates = candidates.intersection(&h2).copied().collect();
            // println!("candidates = {:?}", candidates);
            ret.push(candidates.iter().map(|&x|*x).collect());
        }
    }
    ret
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let lines = vec![
            "abc".to_string(),
            "def".to_string(),
            "".to_string(),
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
            "".to_string(),
            "abc".to_string(),
            "ghi".to_string(),
            "".to_string(),
        ];
        // assert_eq!(task(lines), "{98, 99, 97} {} {} {98, 97, 99} {} {105, 104, 103} {99, 98, 97} {} {} ");
        assert_eq!(
            task(lines),
            vec![
                HashSet::from_iter(vec![98, 99, 97]),
                HashSet::from_iter(vec![]),
                HashSet::from_iter(vec![]),
                HashSet::from_iter(vec![98, 99, 97]),
                HashSet::from_iter(vec![]),
                HashSet::from_iter(vec![105, 104, 103]),
                HashSet::from_iter(vec![99, 98, 97]),
                HashSet::from_iter(vec![]),
                HashSet::from_iter(vec![]),
            ]);
    }
}
