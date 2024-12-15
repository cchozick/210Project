pub mod csv_loader {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn parse_csv(file_path: &str) -> Result<HashMap<String, Vec<(String, usize)>>, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut graph_data: HashMap<String, Vec<(String, usize)>> = HashMap::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if i == 0 {
                continue; 
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 15 {
                let movie = parts[0].trim().to_string();
                let director = parts[8].trim().to_string();
                let star = parts[9].trim().to_string();
                let genre = parts[2].trim().to_string();
                let score: f64 = parts[5].trim().parse().unwrap_or(0.0);
                let votes: f64 = parts[6].trim().parse().unwrap_or(0.0);

                let weight = (score * votes) as usize; 
                graph_data.entry(movie.clone()).or_insert_with(Vec::new).push((director.clone(), weight));
                graph_data.entry(movie.clone()).or_insert_with(Vec::new).push((star.clone(), weight));
                graph_data.entry(movie.clone()).or_insert_with(Vec::new).push((genre.clone(), weight));

                graph_data.entry(director.clone()).or_insert_with(Vec::new).push((movie.clone(), weight));
                graph_data.entry(star.clone()).or_insert_with(Vec::new).push((movie.clone(), weight));
                graph_data.entry(genre.clone()).or_insert_with(Vec::new).push((movie.clone(), weight));
            }
        }

        Ok(graph_data)
    }
}

