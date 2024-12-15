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




