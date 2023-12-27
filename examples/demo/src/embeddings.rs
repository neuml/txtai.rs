use std::error::Error;

use txtai::embeddings::{Document, Embeddings};

/// Example embeddings functionality.
///
/// Implements functionality found in this notebook: https://github.com/neuml/txtai/blob/master/examples/01_Introducing_txtai.ipynb
pub async fn embeddings() -> Result<(), Box<dyn Error>> {
    let embeddings = Embeddings::with_url("http://localhost:8000");

    let mut data = ["US tops 5 million confirmed virus cases",
                    "Canada's last fully intact ice shelf has suddenly collapsed, forming a Manhattan-sized iceberg",
                    "Beijing mobilises invasion craft along coast as Taiwan tensions escalate",
                    "The National Park Service warns against sacrificing slower friends in a bear attack",
                    "Maine man wins $1M from $25 lottery ticket",
                    "Make huge profits without work, earn up to $100,000 a day"];

    let documents: Vec<Document> = data.iter().enumerate().map(|(i, x)| {
        Document { id: i.to_string(), text: x.to_string() }
    }).collect();

    println!("Running similarity queries");
    println!("{:<20} {}", "Query", "Best Match");
    println!("{}", "-".repeat(50));

    for query in ["feel good story", "climate change", "public health story", "war", "wildlife", "asia", "lucky", "dishonest junk"].iter() {
        let results = embeddings.similarity(query, &data.to_vec()).await?;
        let uid = results[0].id;

        println!("{:<20} {}", query, data[uid]);
    }

    embeddings.add(&documents).await?;
    embeddings.index().await?;

    println!("\nBuilding an Embeddings index");
    println!("{:<20} {}", "Query", "Best Match");
    println!("{}", "-".repeat(50));

    for query in ["feel good story", "climate change", "public health story", "war", "wildlife", "asia", "lucky", "dishonest junk"].iter() {
        let results = embeddings.search(query, 1, None, None).await?;
        let uid: usize = results[0].id.parse()?;
        println!("{:<20} {}", query, data[uid]);
    }

    data[0] = "See it: baby panda born";

    let updates: Vec<Document> = vec![Document {id: "0".to_string(), text: data[0].to_string()}];
    embeddings.delete(&["5"].to_vec()).await?;
    embeddings.add(&updates).await?;
    embeddings.upsert().await?;

    println!("\nTest delete/upsert/count");
    println!("{:<20} {}", "Query", "Best Match");
    println!("{}", "-".repeat(50));

    let query = "feel good story";
    let results = embeddings.search(query, 1, None, None).await?;
    let uid: usize = results[0].id.parse()?;
    println!("{:<20} {}", query, data[uid]);

    let count = embeddings.count().await?;
    println!("\nTotal Count: {}", count);

    Ok(())
}
