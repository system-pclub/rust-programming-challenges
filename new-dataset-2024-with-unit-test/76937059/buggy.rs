fn create_stacks() -> Vec<Vec<i32>> {
    let mut stacks = vec![vec![]; 2];
    stacks[0].push(1);
    stacks[0].push(2);
    stacks[1].push(3);
    stacks[1].push(4);
    stacks
}

fn move_top_element(stacks: &mut Vec<Vec<i32>>) {
    let n = stacks[0].pop().unwrap();
    stacks[1].push(n);
}

fn extend_left_stack(stacks: &mut Vec<Vec<i32>>) {
    let mut from = 1;
    let mut to = 0;
    let count = 2;
    // In the real example from, to and count come from a list of N tuples
    // Just limited to two here
    for _ in 0..1 {
        let end = stacks[from].iter().take(count);
        stacks[to].extend(end); // <-- Can't do this as end is an immutable borrow

        from = 0;
        to = 1;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_stacks() {
        let stacks = create_stacks();
        assert_eq!(stacks, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_move_top_element() {
        let mut stacks = create_stacks();
        move_top_element(&mut stacks);
        assert_eq!(stacks, vec![vec![1], vec![3, 4, 2]]);
    }

    #[test]
    fn test_extend_left_stack() {
        let mut stacks = vec![vec![1], vec![3, 4, 2]];
        extend_left_stack(&mut stacks);
        assert_eq!(stacks, vec![vec![1, 3, 4], vec![3, 4, 2]]);
    }
}
