use std::env;
use std::error::Error;

mod embeddings;
mod extractor;
mod labels;
mod pipelines;

use embeddings::embeddings;
use extractor::extractor;
use labels::labels;
use pipelines::pipelines;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} (default)embeddings|extractor|labels|pipelines", args[0])
    }
    else if args[1] == "extractor" {
        extractor().await?;
    }
    else if args[1] == "labels" {
        labels().await?;
    }
    else if args[1] == "pipelines" {
        pipelines().await?;
    }
    else {
        embeddings().await?;
    }

    Ok(())
}
