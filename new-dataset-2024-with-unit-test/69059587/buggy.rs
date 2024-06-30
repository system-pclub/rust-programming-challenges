#[derive(Debug, Clone, PartialEq)]
struct Inner {
    num: usize,
}

#[derive(Debug, PartialEq)]
enum Outer {
    ActionA(Inner),
    ActionB(Inner),
}

fn deterministic_strategy(inners: &Vec<&Inner>) -> Box<dyn FnMut() -> Outer> {
    let mut index = 0;
    Box::new(move || {
        let inner = inners[index % inners.len()];
        index += 1;
        if index % 2 == 0 {
            Outer::ActionA(inner.to_owned().clone())
        } else {
            Outer::ActionB(inner.to_owned().clone())
        }
    })
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_strategy() {
        let inners = vec![&Inner { num: 1 }, &Inner { num: 2 }];
        let mut get_choice = deterministic_strategy(&inners);

        assert_eq!(get_choice(), Outer::ActionB(Inner { num: 1 }));
        assert_eq!(get_choice(), Outer::ActionA(Inner { num: 2 }));
        assert_eq!(get_choice(), Outer::ActionB(Inner { num: 1 }));
        assert_eq!(get_choice(), Outer::ActionA(Inner { num: 2 }));
    }
}

