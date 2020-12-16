use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fmt;

pub use crate::api::API;

pub type APIAnswer = Result<Vec<Answer>, Box<dyn Error>>;

/// Extractor definition
pub struct Extractor {
    api: API
}

/// Extractor implementation
impl Extractor {
    /// Creates an Extractor instance.
    /// 
    /// # Arguments
    /// * `url` - base url of txtai API
    pub fn new(url: &str) -> Extractor {
        Extractor {
            api: API::new(url)
        }
    }

     /// Extracts answers to input questions
     /// 
     /// # Arguments
     /// * `documents` - list of {id: value, text: value}
     /// * `queue` -  list of {name: value, query: value, question: value, snippet: value)
    pub async fn extract(&self, documents: &Vec<Section>, queue: &Vec<Question>) -> APIAnswer {
        // Post parameters
        let params = json!({"documents": documents, "queue": queue});

        // Execute API call
        Ok(self.api.post("extract", &params).await?.json().await?)
    }
}

/// Input section with candidate text for QA process
#[derive(Debug, Serialize)]
pub struct Section {
    pub id: i32,
    pub text: String
}

/// Input Question
#[derive(Debug, Serialize)]
pub struct Question {
    pub name: String,
    pub query: String,
    pub question: String,
    pub snippet: bool
}

/// Answer response
#[derive(Debug, Deserialize)]
pub struct Answer {
    pub question: String,
    pub answer: String
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.question, self.answer)
    }
}
