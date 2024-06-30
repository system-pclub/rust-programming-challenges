struct Tree {
    true_branch: Option<Box<Tree>>,
    false_branch: Option<Box<Tree>>,
}

impl Tree {
    fn get_mut(&mut self, mut directions: impl Iterator<Item = bool>) -> Option<&mut Self> {
        let mut current = self;
        loop {
            match directions.next() {
                None => break,
                Some(true) => {
                    current = current.true_branch.as_deref_mut()?;
                }
                Some(false) => {
                    current = current.false_branch.as_deref_mut()?;
                }
            }
        }
        Some(current)
    }

    fn get_last_initialized_mut(
        &mut self,
        mut directions: impl Iterator<Item = bool>,
    ) -> Result<&mut Self, &mut Self> {
        let mut current = self;
        loop {
            match directions.next() {
                None => break,
                Some(true) => {
                    if let Some(ref mut next) = current.true_branch {
                        current = next;
                    } else {
                        return Err(current);
                    }
                }
                Some(false) => {
                    if let Some(ref mut next) = current.false_branch {
                        current = next;
                    } else {
                        return Err(current);
                    }
                }
            }
        }
        Ok(current)
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut tree = Tree {
            true_branch: Some(Box::new(Tree {
                true_branch: None,
                false_branch: Some(Box::new(Tree {
                    true_branch: None,
                    false_branch: None,
                })),
            })),
            false_branch: Some(Box::new(Tree {
                true_branch: None,
                false_branch: None,
            })),
        };
        let last_initialized = tree.get_last_initialized_mut([true, false].iter().copied());
        assert!(matches!(last_initialized, Ok(_)));
        let last_initialized = tree.get_last_initialized_mut([true, true].iter().copied());
        assert!(matches!(last_initialized, Err(_)));
    }
}
