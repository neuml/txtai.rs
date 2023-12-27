use std::error::Error;

use txtai::extractor::{Extractor, Question};

/// Example extractor functionality.
///
/// Implements logic found in this notebook: https://github.com/neuml/txtai/blob/master/examples/05_Extractive_QA_with_txtai.ipynb
pub async fn extractor() -> Result<(), Box<dyn Error>> {
    let extractor = Extractor::with_url("http://localhost:8000");

    let data = ["Giants hit 3 HRs to down Dodgers",
                "Giants 5 Dodgers 4 final",
                "Dodgers drop Game 2 against the Giants, 5-4",
                "Blue Jays beat Red Sox final score 2-1",
                "Red Sox lost to the Blue Jays, 2-1",
                "Blue Jays at Red Sox is over. Score: 2-1",
                "Phillies win over the Braves, 5-0",
                "Phillies 5 Braves 0 final",
                "Final: Braves lose to the Phillies in the series opener, 5-0",
                "Lightning goaltender pulled, lose to Flyers 4-1",
                "Flyers 4 Lightning 1 final",
                "Flyers win 4-1"];

    // Run series of questions
    let questions = ["What team won the game?", "What was score?"];
    for query in ["Red Sox - Blue Jays", "Phillies - Braves", "Dodgers - Giants", "Flyers - Lightning"].iter() {
        println!("----{}----", query);

        let queue = questions.iter().map(|question| {
            Question { name: question.to_string(), query: query.to_string(), question: question.to_string(), snippet: false }
        }).collect();

        for answer in extractor.extract(&queue, &data.to_vec()).await? {
            println!("{}", answer);
        }
        println!();
    }

    // Ad-hoc questions
    let question = "What hockey team won?";

    println!("----{}----", question);
    let queue = vec![Question {name: question.to_string(), query: question.to_string(), question: question.to_string(), snippet: false}];
    for answer in extractor.extract(&queue, &data.to_vec()).await? {
        println!("{}", answer)
    }

    Ok(())
}
