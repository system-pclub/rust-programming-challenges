#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<V> {
    val: Option<V>,
    children: Vec<(char, Tree<V>)>,
}

fn str_split_first(x: &str) -> Option<(char, &str)> {
    let mut chars = x.chars();
    let first = chars.next()?;
    Some((first, chars.as_str()))
}

impl<V> Tree<V> {
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        let mut current_node: &mut Tree<V> = self;
        let mut key_index: usize = 0;

        loop {
            match str_split_first(&key[key_index..]) {
                Some((c, _)) => {
                    let mut found: bool = false;

                    'child_loop: for (character, child) in current_node.children.iter_mut() {
                        if c == *character {
                            current_node = child;
                            found = true;
                            break 'child_loop;
                        }
                    }

                    // If nothing was found, this key does not exist.
                    if found == false {
                        return None;
                    }

                    key_index += 1;
                }
                _ => break,
            }
        }

        let val_opt: Option<&mut V> = current_node.val.as_mut();
        return match val_opt {
            Some(val) => return Some(val),
            None => None,
        };
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mut() {
        let mut tree = Tree {
            val: None,
            children: vec![
                ('a', Tree {
                    val: None,
                    children: vec![
                        ('b', Tree {
                            val: Some(10),
                            children: vec![],
                        }),
                    ],
                }),
            ],
        };

        assert_eq!(tree.get_mut("ab"), Some(&mut 10));
        assert_eq!(tree.get_mut("a"), None);
        assert_eq!(tree.get_mut("b"), None);
        assert_eq!(tree.get_mut("abc"), None);
    }
}