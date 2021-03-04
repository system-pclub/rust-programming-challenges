use std::collections::HashMap;

fn main() {
    let mut bucket: HashMap<&str, Vec<&str>> = HashMap::new();
    bucket.insert("a", vec!["hello", "good morning"]);
    bucket.insert("b", vec!["bye", "ciao"]);
    bucket.insert("c", vec!["good"]);
    let cluster = vec!["a", "b"];
    let cluster2 = vec!["c"];
    let mut clusters = [cluster, cluster2];
    clusters.iter_mut().for_each(|cluster| {
        // I don't like this clone
        let tmp = cluster.clone();
        let tmp = tmp.iter().flat_map(|seq| bucket.remove(seq).unwrap());
        cluster.extend(tmp);
    });
    println!("{:?}", clusters);
}