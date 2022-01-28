use std::rc::Rc;
#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Rc<Node>>,
    pub edges: Vec<Edge>
}
#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub label: String
}
#[derive(Debug)]
pub struct Edge {
    pub source: Rc<Node>,
    pub target: Rc<Node>
}

impl Graph {
    pub fn from_json(json: & String) -> Graph {
        if let json::JsonValue::Object(deserialized) = json::parse(json.as_ref()).unwrap() {
            let nodes : Vec<Rc<Node>> = deserialized.get("nodes").unwrap().members()
                .map(|v| {
                    if let json::JsonValue::Object(ref val) = *v {
                        return Rc::new(Node {
                            id: val.get("id").unwrap().to_string(),
                            label: val.get("label").unwrap().to_string()
                        });
                    }
                    panic!("Invalid structure of json graph body.")
            }).collect::<Vec<Rc<Node>>>();
            let edges : Vec<Edge> = deserialized.get("edges").unwrap().members()
                .map(|v| {
                    if let json::JsonValue::Object(ref val) = *v {
                        let source = nodes.iter().find(|&v| v.id ==  val.get("source").unwrap().to_string()).unwrap();
                        let target = nodes.iter().find(|&v| v.id ==  val.get("target").unwrap().to_string()).unwrap();
                        return Edge {
                            source: Rc::clone(&source),
                            target: Rc::clone(&target)
                        };
                    }
                    panic!("Invalid structure of json graph body.")
                }).collect::<Vec<Edge>>();
            return Graph {
                nodes,
                edges
            }
        }
        panic!("Incorrect struct of json contains!");
    }
}


fn main() {}