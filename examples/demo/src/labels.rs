use std::error::Error;

use txtai::labels::Labels;

/// Example labels functionality.
///
/// Implements logic found in this notebook: https://github.com/neuml/txtai/blob/master/examples/07_Apply_labels_with_zero_shot_classification.ipynb
pub async fn labels() -> Result<(), Box<dyn Error>> {
    let labels = Labels::with_url("http://localhost:8000");

    let data = ["Dodgers lose again, give up 3 HRs in a loss to the Giants",
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

    for text in data.iter() {
        let tags = vec!["Baseball", "Football", "Hockey", "Basketball"];
        let label = labels.label(text, &tags.to_vec()).await?[0].id;

        println!("{:<75} {}", text, tags[label]);
    }

    println!("");
    println!("{:<75} {}", "Text", "Label");
    println!("{}", "-".repeat(100));

    for text in data.iter() {
        let tags = vec!["😀", "😡"];
        let label = labels.label(text, &tags).await?[0].id;

        println!("{:<75} {}", text, tags[label]);
    }

    Ok(())
}
