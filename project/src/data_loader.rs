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

        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() <= 7 {
            eprintln!("Skipping row {}: not enough columns", i + 1);
            continue;
        }

        let directors = parts[6].trim();
        let actors = parts[7].trim();

        let directors_list: Vec<&str> = directors.split(';').collect();
        let actors_list: Vec<&str> = actors.split(';').collect();

        let mut participants = Vec::new();
        for &name in directors_list.iter().chain(actors_list.iter()) {
            let node = *name_to_node.entry(name.to_string()).or_insert_with(|| {
                graph.add_node(name.to_string())
            });
            participants.push((name, node));
        }

        for (_, node) in &participants {
            for (_, other_node) in &participants {
                if node != other_node {
                    let key = if *node < *other_node {
                        (*node, *other_node)
                    } else {
                        (*other_node, *node)
                    };
                    *collaborations.entry(key).or_insert(0) += 1;

                    if graph.find_edge(*node, *other_node).is_none() {
                        graph.add_edge(*node, *other_node, 1);
                    }
                }
            }
        }
    }

    (graph, name_to_node)
}

