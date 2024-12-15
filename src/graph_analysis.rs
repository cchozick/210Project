pub mod graph_analysis {
    use std::collections::{HashMap, HashSet};

    pub struct Graph {
        pub adjacency_list: HashMap<String, Vec<(String, usize)>>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                adjacency_list: HashMap::new(),
            }
        }

        pub fn from_data(data: HashMap<String, Vec<(String, usize)>>) -> Self {
            Self {
                adjacency_list: data,
            }
        }

        pub fn add_node(&mut self, node: String) {
            self.adjacency_list.entry(node).or_insert(Vec::new());
        }

        pub fn add_edge(&mut self, from: String, to: String, weight: usize) {
            self.adjacency_list
                .get(&movie) 
                .and_then(|neighbors| {
                    neighbors.iter().find(|(node, _)| {
                        node.as_str() == actor || node.as_str() == director
                    })
                })
                .map(|(_, weight)| *weight as f64)


        pub fn analyze_connected_nodes(&self, node: &str) {
            if let Some(neighbors) = self.adjacency_list.get(node) {
                println!("\nConnections for '{}':", node);
                for (neighbor, weight) in neighbors {
                    println!("- {} (weight: {})", neighbor, weight);
                }
            } else {
                println!("\nNode '{}' not found in the graph.", node);
            }
        }
    }
    
        pub fn calculate_total_weight(&self, node: &str) -> usize {
            if let Some(neighbors) = self.adjacency_list.get(node) {
                neighbors.iter().map(|(_, weight)| weight).sum()
            } else {
                0
            }
        }

        pub fn detect_communities(&self) -> Vec<Vec<String>> {
            let mut visited = HashSet::new();
            let mut communities = Vec::new();

            for node in self.adjacency_list.keys() {
                if !visited.contains(node) {
                    let mut community = Vec::new();
                    self.dfs_collect(node, &mut visited, &mut community);
                    communities.push(community);
                }
            }

            communities
        }

        fn dfs_collect(&self, node: &str, visited: &mut HashSet<String>, community: &mut Vec<String>) {
            if visited.contains(node) {
                return;
            }

            visited.insert(node.to_string());
            community.push(node.to_string());

            if let Some(neighbors) = self.adjacency_list.get(node) {
                for (neighbor, _) in neighbors {
                    self.dfs_collect(neighbor, visited, community);
                }
            }
        }

        pub fn predict_collaboration(&self, actor: &str, director: &str) -> Option<f64> {
            let actor_movies: HashSet<_> = self.adjacency_list.get(actor)?.iter().map(|(movie, _)| movie).collect();
            let director_movies: HashSet<_> = self.adjacency_list.get(director)?.iter().map(|(movie, _)| movie).collect();

            let shared_movies: Vec<_> = actor_movies.intersection(&director_movies).collect();

            if shared_movies.is_empty() {
                println!("No previous collaborations found between '{}' and '{}'.", actor, director);
                return None;
            }

            let avg_rating: f64 = shared_movies.iter().filter_map(|&movie| {
                self.adjacency_list.get(&movie)?.iter().find(|(node, _)| node.as_str() == actor || node.as_str() == director).map(|(_, weight)| *weight as f64)
            }).sum::<f64>() / shared_movies.len() as f64;

            println!("Previous collaborations between '{}' and '{}':", actor, director);
            for movie in &shared_movies {
                println!("- {}", movie);
            }

            println!("Average rating of shared movies: {:.2}", avg_rating);
            Some(avg_rating)
        }
    }
}
