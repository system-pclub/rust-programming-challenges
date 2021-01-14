
pub struct Graph<'a> {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge<'a>>,
}

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub label: String,
}

pub struct Edge<'a> {
    pub source: &'a Node,
    pub target: &'a Node,
}

impl<'a> Graph<'a> {
    pub fn from_json(json: &String) -> Graph {
        if let json::JsonValue::Object(deserialized) = json::parse(json.as_ref()).unwrap() {
            let nodes: Vec<Node> = deserialized
                .get("nodes")
                .unwrap()
                .members()
                .map(|v| {
                    if let json::JsonValue::Object(ref val) = *v {
                        return Node {
                            id: val.get("id").unwrap().to_string(),
                            label: val.get("label").unwrap().to_string(),
                        };
                    }
                    panic!("Invalid structure of json graph body.")
                })
                .collect::<Vec<Node>>();
            let edges: Vec<Edge> = deserialized
                .get("edges")
                .unwrap()
                .members()
                .map(|v| {
                    if let json::JsonValue::Object(ref val) = *v {
                        let source = (*nodes)
                            .iter()
                            .find(|&v| v.id == val.get("source").unwrap().to_string())
                            .unwrap();
                        let target = (*nodes)
                            .iter()
                            .find(|&v| v.id == val.get("target").unwrap().to_string())
                            .unwrap();
                        return Edge { source, target };
                    }
                    panic!("Invalid structure of json graph body.")
                })
                .collect::<Vec<Edge>>();
            return Graph { nodes, edges };
        }
        panic!("Incorrect struct of json contains!");
    }
}

fn main() {}