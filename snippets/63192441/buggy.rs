#[derive(Debug)]
struct Node {
    value: u64,
    children: Vec<Node>
}

fn run_loop<F>(mut handler: F) 
where F: 'static + FnMut() {
    for _ in 0..500 {
        handler();
    }
}

fn main() {
    let nodes = vec![
        Node {
            value: 1,
            children: vec![
                Node { value: 3, children: vec![] }
            ],
        },
        Node {
            value: 2,
            children: vec![],
        },
    ];
    let mut node = &nodes[0];
    
    run_loop(move || {
        println!("Node: {:?}", node);
        node = &node.children[0];
    });
}