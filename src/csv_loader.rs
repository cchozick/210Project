use crate::graph_analysis::CollaborationGraph;
use std::error::Error;


pub fn populate_graph_from_csv(file_path: &str, graph: &mut CollaborationGraph) -> Result<(), Box<dyn Error>> {
   let mut reader = csv::Reader::from_path(file_path)?;


   for result in reader.records() {
       let record: csv::StringRecord = result?;
      
       let _movie = record.get(0).unwrap_or("").to_lowercase();
       let director = record.get(7).unwrap_or("").to_lowercase();
       let star = record.get(9).unwrap_or("").to_lowercase();
       let score: f64 = record.get(5).unwrap_or("0.0").parse().unwrap_or(0.0);


       if director.is_empty() {
           eprintln!("Skipping row due to missing director: {:?}", record);
           continue;
       }
       if star.is_empty() {
           eprintln!("Skipping row due to missing actor: {:?}", record);
           continue;
       }


       println!(
           "Adding collaboration: Actor = {}, Director = {}, Score = {:.2}",
           star, director, score
       );


       graph.add_edge(&star, &director, score);
   }


   Ok(())
}
