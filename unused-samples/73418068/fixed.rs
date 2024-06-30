use std::io::stdin;

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
enum Path {
    Left,
    Right,
}

fn to_vec(node: &Node, data: &mut Vec<i32>) {
    // recurse left child
    if let Some(left_child) = &node.left {
        to_vec(&left_child, data);
    }

    // current node
    data.push(node.value);

    // recurse right child
    if let Some(right_child) = &node.right {
        to_vec(&right_child, data)
    }
}

#[derive(Debug)]
enum Error {
    DuplicateValue,
    NotFound,
    EmptyTree,
}

impl Tree {
    fn insert(&mut self, value: i32) -> Result<(), Error> {
        if let Some(root) = &mut self.root {
            let mut current_node = root;
            loop {
                if current_node.value == value {
                    return Err(Error::DuplicateValue);
                } else if value > current_node.value {
                    let right = &mut current_node.right;
                    if let Some(right) = right {
                        current_node = right;
                        continue;
                    } else {
                        *right = Some(Box::new(Node {
                            value,
                            left: None,
                            right: None,
                        }));
                        return Ok(());
                    }
                } else {
                    let left = &mut current_node.left;
                    if let Some(left) = left {
                        current_node = left;
                        continue;
                    } else {
                        *left = Some(Box::new(Node {
                            value,
                            left: None,
                            right: None,
                        }));
                        return Ok(());
                    }
                }
            }
        } else {
            self.root = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
            return Ok(());
        }
    }

    fn find(&self, value: i32) -> Result<Vec<Path>, Error> {
        let mut p: Vec<Path> = vec![];
        if let Some(root) = &self.root {
            let mut current_node = root;
            loop {
                if current_node.value == value {
                    return Ok(p);
                } else if value > current_node.value {
                    let right = &current_node.right;
                    if let Some(right) = right {
                        p.push(Path::Right);
                        current_node = right;
                        continue;
                    } else {
                        return Err(Error::NotFound);
                    }
                } else {
                    let left = &current_node.left;
                    if let Some(left) = left {
                        p.push(Path::Left);
                        current_node = left;
                        continue;
                    } else {
                        return Err(Error::NotFound);
                    }
                }
            }
        } else {
            return Err(Error::EmptyTree);
        }
    }

    fn left(&mut self) -> Result<(), Error> {
        match self.root.as_mut() {
            Some(root) => match root.left {
                Some(left) => {
                    self.root = Some(left);
                    return Ok(());
                }
                None => {
                    return Err(Error::NotFound);
                }
            },
            None => {
                return Err(Error::EmptyTree);
            }
        }
    }
}

impl Error {
    fn to_string(self) -> String {
        use Error::*;
        match self {
            NotFound => "NotFound".to_string(),
            EmptyTree => "EmptyTree".to_string(),
            DuplicateValue => "DuplicateValue".to_string(),
        }
    }
}

fn main() {
    let mut tree = Tree { root: None };
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    let a: i32 = a.trim().parse().unwrap();
    for _ in 0..a {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let commands: Vec<&str> = line.trim().split(' ').collect();
        match commands[0] {
            "L" => {
                if let Err(er) = tree.left() {
                    println!("{}", er.to_string());
                }
            }
            "I" => {
                let a: i32 = commands[1].parse().unwrap();
                if let Err(er) = tree.insert(a) {
                    println!("{}", er.to_string());
                }
            }
            "P" => {
                let mut res = vec![];
                if let Some(root) = &tree.root {
                    to_vec(&root, &mut res);
                    println!("{:?}", res);
                } else {
                    println!("EmptyTree");
                }
            }
            "F" => {
                let a: i32 = commands[1].parse().unwrap();
                if let Err(er) = tree.find(a) {
                    println!("{}", er.to_string());
                } else {
                    println!("{:?}", tree.find(a).unwrap());
                }
            }
            _ => unreachable!(""),
        }
    }
}
