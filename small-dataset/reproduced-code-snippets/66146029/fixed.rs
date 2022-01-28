use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let mut candidates = HashSet::<&u8>::new();
    let lines = vec!["hello".to_string()];
    let mut sum = 0;
    for l in lines.iter() { // lines is Vec<String>
        if candidates.len()==0 {
            candidates = HashSet::<&u8>::from_iter(l.as_bytes());
            println!("candidates = {:?}", candidates);
        } else if l.len() == 0 { // error message "borrow later used here"
            // end of group
            sum = sum + candidates.len();
            candidates = HashSet::<&u8>::new();
        } else {
            let h2 = HashSet::<&u8>::from_iter(l.as_bytes()); // error message "borrowed value does not live long enough"
            candidates = candidates.intersection(&h2).copied().collect();
            println!("candidates = {:?}", candidates);
        }
        println!("{}", l);
    }
}
