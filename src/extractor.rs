use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fmt;

pub use crate::api::API;

/// Extractor definition
pub struct Extractor {
    api: API
}

/// Extractor implementation
impl Extractor {
    /// Creates an Extractor instance.
    /// 
    pub fn new() -> Extractor {
        Extractor { 
            api: API::new()
        }
    }

    /// Creates an Extractor instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    pub fn with_url(url: &str) -> Extractor {
        Extractor {
            api: API::with_url(url) 
        }
    }

    /// Creates an API instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    /// * `token` - API token
    pub fn with_url_token(url: &str, token: &str) -> Extractor {
        Extractor {
            api: API::with_url_token(url, token)
        }
    }

     /// Extracts answers to input questions.
     /// 
     /// # Arguments
     /// * `queue` -  list of {name: value, query: value, question: value, snippet: value}
     /// * `texts` - list of texts
    pub async fn extract(&self, queue: &Vec<Question>, texts: &Vec<&str>) -> Answers {
        // Post parameters
        let params = json!({"queue": queue, "texts": texts});

        // Execute API call
        Ok(self.api.post("extract", &params).await?.json().await?)
    }
}

// Extractor return types
pub type Answers = Result<Vec<Answer>, Box<dyn Error>>;

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
    pub name: String,
    pub answer: String
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.answer)
    }
}
