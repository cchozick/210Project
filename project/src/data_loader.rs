use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_graph_from_csv(file_path: &str) -> (Graph<String, u32, Undirected>, HashMap<String, NodeIndex>) {
    let file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Error opening file {}: {}", file_path, err);
        std::process::exit(1);
    });
    let reader = BufReader::new(file);

    let mut graph = Graph::<String, u32, Undirected>::new_undirected();
    let mut name_to_node: HashMap<String, NodeIndex> = HashMap::new();
    let mut collaborations: HashMap<(NodeIndex, NodeIndex), u32> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
