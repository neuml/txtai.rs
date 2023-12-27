use serde_json::json;

pub use crate::api::{API, Texts, TextsBatch};

/// Textractor definition
pub struct Textractor {
    api: API
}

/// Textractor implementation
impl Textractor {
    /// Creates a Textractor instance.
    /// 
    pub fn new() -> Textractor {
        Textractor { 
            api: API::new()
        }
    }

    /// Creates a Textractor instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    pub fn with_url(url: &str) -> Textractor {
        Textractor {
            api: API::with_url(url) 
        }
    }

    /// Creates a Textractor instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    /// * `token` - API token
    pub fn with_url_token(url: &str, token: &str) -> Textractor {
        Textractor {
            api: API::with_url_token(url, token)
        }
    }

    /// Extracts text from a file at path.
    ///  
    /// # Arguments
    /// * `file` file to extract text
    pub async fn textract(&self, file: &str) -> Texts {
        // Query parameters
        let params = [("file", file)];

        // Execute API call
        Ok(self.api.get("textract", &params).await?.json().await?)
    }

    /// Extracts text from a file at path.
    ///  
    /// # Arguments
    /// * `files` file to extract text
    pub async fn batchtextract(&self, files: &Vec<&str>) -> TextsBatch {
        // Post parameters
        let params = json!(files);

        // Execute API call
        Ok(self.api.post("batchtextract", &params).await?.json().await?)
    }
}
