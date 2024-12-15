use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;


#[derive(Debug)]
pub struct CollaborationGraph {
   graph: Graph<String, f64, Undirected>,
   node_map: HashMap<String, NodeIndex>,
}


impl CollaborationGraph {
   pub fn new() -> Self {
       Self {
           graph: Graph::new_undirected(),
           node_map: HashMap::new(),
       }
   }


   fn add_node(&mut self, name: &str) -> NodeIndex {
       if let Some(&index) = self.node_map.get(name) {
           index
       } else {
           let index = self.graph.add_node(name.to_string());
           self.node_map.insert(name.to_string(), index);
           index
       }
   }


   pub fn add_edge(&mut self, actor: &str, director: &str, rating: f64) {
       let actor_index = self.add_node(actor);
       let director_index = self.add_node(director);
       self.graph.add_edge(actor_index, director_index, rating);
   }


   pub fn predict_rating(&self, actor: &str, director: &str) -> Option<f64> {
       let actor = actor.to_lowercase();
       let director = director.to_lowercase();
  
       let actor_index = self.node_map.get(&actor)?;
       let director_index = self.node_map.get(&director)?;
  
       let actor_ratings: Vec<f64> = self
           .graph
           .edges(*actor_index)
           .map(|edge| *edge.weight())
           .collect();
  
       let director_ratings: Vec<f64> = self
           .graph
           .edges(*director_index)
           .map(|edge| *edge.weight())
           .collect();
  
       let actor_avg = if !actor_ratings.is_empty() {
           actor_ratings.iter().sum::<f64>() / actor_ratings.len() as f64
       } else {
           0.0
       };
  
       let director_avg = if !director_ratings.is_empty() {
           director_ratings.iter().sum::<f64>() / director_ratings.len() as f64
       } else {
           0.0
       };
  
       Some((actor_avg + director_avg) / 2.0)
   }
}
