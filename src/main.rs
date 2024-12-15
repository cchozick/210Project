mod csv_loader;
mod graph_analysis;

use csv_loader::populate_graph_from_csv;
use graph_analysis::CollaborationGraph;
use std::io::{self, Write};


fn main() {
   let file_path = "movies.csv";
   let mut graph = CollaborationGraph::new();


   if let Err(err) = populate_graph_from_csv(file_path, &mut graph) {
       eprintln!("Failed to load graph: {}", err);
       return;
   }


   println!("Graph successfully populated!");


   loop {
       let mut actor_name = String::new();
       let mut director_name = String::new();


       print!("\nEnter the name of the actor (or 'exit' to quit): ");
       io::stdout().flush().unwrap();
       if io::stdin().read_line(&mut actor_name).is_err() {
           println!("Error reading input. Please try again.");
           continue;
       }
       let actor_name = actor_name.trim().to_lowercase();
       if actor_name.eq_ignore_ascii_case("exit") {
           println!("Exiting program");
           break;
       }


       print!("Enter the name of the director: ");
       io::stdout().flush().unwrap();
       if io::stdin().read_line(&mut director_name).is_err() {
           println!("Error reading input. Try again.");
           continue;
       }
       let director_name = director_name.trim().to_lowercase();


       match graph.predict_rating(&actor_name, &director_name) {
           Some(predicted_rating) => {
               println!(
                   "Predicted rating for collaboration between '{}' and '{}': {:.2}",
                   actor_name, director_name, predicted_rating
               );
           }
           None => {
               println!(
                   "No collaboration data found for '{}' and '{}'.",
                   actor_name, director_name
               );
           }
       }
   }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incomplete_data_handling() {
        let mut graph = CollaborationGraph::new();

        let data = vec![
            ("", "Director A", 7.5), 
            ("Actor B", "", 6.0),  
            ("Actor C", "Director C", 0.0),
        ];

        for (actor, director, score) in data {
            if actor.is_empty() || director.is_empty() {
                continue;
            }
            graph.add_edge(actor, director, score);
        }

        assert!(graph.node_map.is_empty(), "Graph should not contain nodes for invalid data");
    }

    #[test]
    fn test_non_existent_nodes() {
        let mut graph = CollaborationGraph::new();

        graph.add_edge("Actor A", "Director A", 7.5);

        let result = graph.predict_rating("NonExistentActor", "NonExistentDirector");

        assert!(result.is_none(), "Prediction should return None for non-existent nodes");
    }

    #[test]
    fn test_valid_prediction() {
        let mut graph = CollaborationGraph::new();

        graph.add_edge("Actor E", "Director E", 7.0);
        graph.add_edge("Actor E", "Director F", 6.5);
        graph.add_edge("Actor F", "Director E", 8.0);

       
        let expected = (6.75 + 7.5) / 2.0;

        let result = graph.predict_rating("Actor E", "Director E");

        assert!(result.is_some(), "Prediction should not return None for valid nodes");
        assert!(
            (result.unwrap() - expected).abs() < 0.01,
            "Predicted rating is not within acceptable error range"
        );
    }
}
