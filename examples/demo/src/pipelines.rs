use std::error::Error;

use txtai::segmentation::Segmentation;
use txtai::summary::Summary;
use txtai::textractor::Textractor;
use txtai::transcription::Transcription;
use txtai::translation::Translation;
use txtai::workflow::Workflow;

/// Example pipeline and workflow functionality.
/// 
/// Uses files from txtai unit tests: https://github.com/neuml/txtai/releases/download/v2.0.0/tests.tar.gz
pub async fn pipelines() -> Result<(), Box<dyn Error>> {
    let service = "http://localhost:8000";

    let segment = Segmentation::new(service);

    let sentences = "This is a test. And another test.";

    println!("---- Segmented Text ----");
    println!("{:?}", segment.segment(sentences).await?);

    let textractor = Textractor::new(service);
    let text = textractor.textract("/tmp/txtai/article.pdf").await?;

    println!("\n---- Extracted Text ----");
    println!("{:?}", text);

    let summary = Summary::new(service);
    let summarytext = summary.summary(text.as_string().unwrap(), None, None).await?;

    println!("\n---- Summary Text ----");
    println!("{:?}", summarytext);

    let translate = Translation::new(service);
    let translation = translate.translate(&summarytext, Some("es"), None).await?;

    println!("\n---- Summary Text in Spanish ----");
    println!("{:?}", translation);

    let workflow = Workflow::new(service);
    let output = workflow.workflow("sumspanish", &vec!["file:///tmp/txtai/article.pdf"]).await?;

    println!("\n---- Workflow [Extract Text->Summarize->Translate] ----");
    println!("{:?}", output);

    let transcribe = Transcription::new(service);
    let transcription = transcribe.transcribe("/tmp/txtai/Make_huge_profits.wav").await?;

    println!("\n---- Transcribed Text ----");
    println!("{:?}", transcription);

    Ok(())
}
