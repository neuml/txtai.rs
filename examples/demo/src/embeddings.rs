use std::error::Error;

use txtai::embeddings::{Document, Embeddings};

/// Gets the index of the maximum value found in vec.
/// 
/// # Arguments
/// * `vec` - list of floats to search
fn argmax(vec: Vec<f32>) -> usize {
    // Get best match
    let mut max = -1.0;
    let mut argmax = 0;
    for (i, x) in vec.iter().enumerate() {
        if x > &max {
            argmax = i;
            max = *x;
        }
    }

    return argmax
}

/// Example embeddings functionality.
/// 
/// Implements functionality found in this notebook: https://github.com/neuml/txtai/blob/master/examples/01_Introducing_txtai.ipynb
pub async fn embeddings() -> Result<(), Box<dyn Error>> {
    let embeddings = Embeddings::new("http://localhost:8000");

    let sections = ["US tops 5 million confirmed virus cases", 
                    "Canada's last fully intact ice shelf has suddenly collapsed, forming a Manhattan-sized iceberg",
                    "Beijing mobilises invasion craft along coast as Taiwan tensions escalate",
                    "The National Park Service warns against sacrificing slower friends in a bear attack",
                    "Maine man wins $1M from $25 lottery ticket",
                    "Make huge profits without work, earn up to $100,000 a day"];

    let documents: Vec<Document> = sections.iter().enumerate().map(|(i, x)| {
        Document { id: i.to_string(), text: x.to_string() }
    }).collect();

    println!("Running similarity queries");
    println!("{:<20} {}", "Query", "Best Match");
    println!("{}", "-".repeat(50));

    for query in ["feel good story", "climate change", "health", "war", "wildlife", "asia", "north america", "dishonest junk"].iter() {
        let results = embeddings.similarity(query, &sections.to_vec()).await?;
        let argmax = argmax(results);

        println!("{:<20} {}", query, sections[argmax]);
    }

    embeddings.add(&documents).await?;
    embeddings.index().await?;

    println!("\nBuilding an Embeddings index");
    println!("{:<20} {}", "Query", "Best Match");
    println!("{}", "-".repeat(50));

    for query in ["feel good story", "climate change", "health", "war", "wildlife", "asia", "north america", "dishonest junk"].iter() {
        let results = embeddings.search(query, 1).await?;
        let argmax: usize = results[0].id.parse()?;
        println!("{:<20} {}", query, sections[argmax]);
    }

    Ok(())
}
