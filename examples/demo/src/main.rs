use std::env;
use std::error::Error;

use txtai::embeddings::{Document, Embeddings};
use txtai::extractor::{Extractor, Section, Question};
use txtai::labels::Labels;

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
async fn embeddings() -> Result<(), Box<dyn Error>> {
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

/// Example extractor functionality.
/// 
/// Implements logic found in this notebook: https://github.com/neuml/txtai/blob/master/examples/02_Extractive_QA_with_txtai.ipynb
async fn extractor() -> Result<(), Box<dyn Error>> {
    let extractor = Extractor::new("http://localhost:8000");

    let sections = ["Giants hit 3 HRs to down Dodgers",
                    "Giants 5 Dodgers 4 final",
                    "Dodgers drop Game 2 against the Giants, 5-4",
                    "Blue Jays 2 Red Sox 1 final",
                    "Red Sox lost to the Blue Jays, 2-1",
                    "Blue Jays at Red Sox is over. Score: 2-1",
                    "Phillies win over the Braves, 5-0",
                    "Phillies 5 Braves 0 final",
                    "Final: Braves lose to the Phillies in the series opener, 5-0",
                    "Final score: Flyers 4 Lightning 1",
                    "Flyers 4 Lightning 1 final",
                    "Flyers win 4-1"];

    let documents: Vec<Section> = sections.iter().enumerate().map(|(i, x)| {
        Section { id: i as i32, text: x.to_string() }
    }).collect();

    // Run series of questions
    let questions = ["What team won the game?", "What was score?"];
    for query in ["Red Sox - Blue Jays", "Phillies - Braves", "Dodgers - Giants", "Flyers - Lightning"].iter() {
        println!("----{}----", query);

        let queue = questions.iter().map(|question| {
            Question { name: question.to_string(), query: query.to_string(), question: question.to_string(), snippet: false }
        }).collect();

        for answer in extractor.extract(&documents, &queue).await? {
            println!("{}", answer);
        }
        println!();
    }

    // Ad-hoc questions
    let question = "What hockey team won?";

    println!("----{}----", question);
    let queue = vec![Question {name: question.to_string(), query: question.to_string(), question: question.to_string(), snippet: false}];
    for answer in extractor.extract(&documents, &queue).await? {
        println!("{}", answer)
    }

    Ok(())
}

/// Example labels functionality.
/// 
/// Implements logic found in this notebook: https://github.com/neuml/txtai/blob/master/examples/05_Labeling_with_zero_shot_classification.ipynb
async fn labels() -> Result<(), Box<dyn Error>> {
    let labels = Labels::new("http://localhost:8000");

    let sections = ["Dodgers lose again, give up 3 HRs in a loss to the Giants",
                    "Giants 5 Cardinals 4 final in extra innings",
                    "Dodgers drop Game 2 against the Giants, 5-4",
                    "Flyers 4 Lightning 1 final. 45 saves for the Lightning.",
                    "Slashing, penalty, 2 minute power play coming up",
                    "What a stick save!",
                    "Leads the NFL in sacks with 9.5",
                    "UCF 38 Temple 13",
                    "With the 30 yard completion, down to the 10 yard line",
                    "Drains the 3pt shot!!, 0:15 remaining in the game",
                    "Intercepted! Drives down the court and shoots for the win",
                    "Massive dunk!!! they are now up by 15 with 2 minutes to go"];

    println!("{:<75} {}", "Text", "Label");
    println!("{}", "-".repeat(100));

    for text in sections.iter() {
        let tags = vec!["Baseball", "Football", "Hockey", "Basketball"];
        let label = &labels.label(text, &tags.to_vec()).await?[0].id;

        println!("{:<75} {}", text, label);
    }

    println!("");
    println!("{:<75} {}", "Text", "Label");
    println!("{}", "-".repeat(100));

    for text in sections.iter() {
        let tags = vec!["😀", "😡"];
        let label = &labels.label(text, &tags).await?[0].id;

        println!("{:<75} {}", text, label);
    }

    Ok(())
}

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
