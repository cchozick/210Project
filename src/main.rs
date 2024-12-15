mod csv_loader;
mod graph_analysis;

use crate::graph_analysis::graph_analysis::Graph;
use crate::csv_loader::parse_csv;
use std::io;

fn main() {
    let graph_data = match parse_csv("movies.csv") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error loading CSV: {}", err);
            return;
        }
    };

    let graph = Graph::from_data(graph_data);
    println!("Graph successfully created from CSV.");

    let mut actor = String::new();
    let mut director = String::new();

    println!("\nEnter the name of an actor:");
    if let Ok(_) = io::stdin().read_line(&mut actor) {
        actor = actor.trim().to_string();
    } else {
        println!("Error reading actor input.");
        return;
    }

    println!("\nEnter the name of a director:");
    if let Ok(_) = io::stdin().read_line(&mut director) {
        director = director.trim().to_string();
    } else {
        println!("Error reading director input.");
        return;
    }

    if let Some(predicted_rating) = graph.predict_collaboration(&actor, &director) {
        println!("Predicted success of a movie with '{}' and '{}': {:.2}", actor, director, predicted_rating);
    } else {
        println!("No prediction available for the given actor and director.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_csv;

    #[test]
    fn test_analyze_connected_nodes() {
        let mut graph_data = std::collections::HashMap::new();
        graph_data.entry("Node1".to_string()).or_insert_with(Vec::new).push(("Node2".to_string(), 3));
        graph_data.entry("Node1".to_string()).or_insert_with(Vec::new).push(("Node3".to_string(), 7));
    
        let graph = Graph::from_data(graph_data);
    
        let output = Vec::new();
        let mut cursor = std::io::Cursor::new(output);
    
        let node = "Node1";
        graph.analyze_connected_nodes(node);
    
        assert_eq!(
            format!("Connections for '{}':\n- Node2 (weight: 3)\n- Node3 (weight: 7)\n", node),
            cursor.get_ref().to_string()
        );
    }

    #[test]
    fn test_predict_collaboration() {
        let mut graph_data = std::collections::HashMap::new();
        graph_data.entry("Actor1".to_string()).or_insert_with(Vec::new).push(("Movie1".to_string(), 8));
        graph_data.entry("Actor1".to_string()).or_insert_with(Vec::new).push(("Movie2".to_string(), 9));
        graph_data.entry("Director1".to_string()).or_insert_with(Vec::new).push(("Movie1".to_string(), 8));
        graph_data.entry("Director1".to_string()).or_insert_with(Vec::new).push(("Movie3".to_string(), 7));

        let graph = Graph::from_data(graph_data);

        let predicted_rating = graph.predict_collaboration("Actor1", "Director1");
        assert!(predicted_rating.is_some());
        assert_eq!(predicted_rating.unwrap(), 8.0); 
    }

#[test]
    fn test_detect_communities() {
        let mut graph_data = std::collections::HashMap::new();
        graph_data.entry("Node1".to_string()).or_insert_with(Vec::new).push(("Node2".to_string(), 1));
        graph_data.entry("Node2".to_string()).or_insert_with(Vec::new).push(("Node1".to_string(), 1));
        graph_data.entry("Node3".to_string()).or_insert_with(Vec::new).push(("Node4".to_string(), 1));
        graph_data.entry("Node4".to_string()).or_insert_with(Vec::new).push(("Node3".to_string(), 1));

        let graph = Graph::from_data(graph_data);
        let communities = graph.detect_communities();

        assert_eq!(communities.len(), 2);
        assert!(communities.contains(&vec!["Node1".to_string(), "Node2".to_string()]));
        assert!(communities.contains(&vec!["Node3".to_string(), "Node4".to_string()]));
    }
}