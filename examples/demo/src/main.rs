use std::env;
use std::error::Error;

mod embeddings;
mod extractor;
mod labels;

use embeddings::embeddings;
use extractor::extractor;
use labels::labels;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} (default)embeddings|extractor|labels", args[0])
    }
    else if args[1] == "extractor" {
        extractor().await?;
    }
    else if args[1] == "labels" {
        labels().await?;
    }
    else {
        embeddings().await?;
    }

    Ok(())
}
