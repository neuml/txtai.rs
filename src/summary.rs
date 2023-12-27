use serde_json::json;

pub use crate::api::{API, Strings, StringsBatch};

/// Summary definition
pub struct Summary {
    api: API
}

/// Summary implementation
impl Summary {
    /// Creates a Summary instance.
    /// 
    pub fn new() -> Summary {
        Summary { 
            api: API::new()
        }
    }

    /// Creates a Summary instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    pub fn with_url(url: &str) -> Summary {
        Summary {
            api: API::with_url(url) 
        }
    }

    /// Creates a Summary instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    /// * `token` - API token
    pub fn with_url_token(url: &str, token: &str) -> Summary {
        Summary {
            api: API::with_url_token(url, token)
        }
    }

    /// Runs a summarization model against a block of text.
    /// 
    /// # Arguments
    /// * `text` text to summarize
    /// * `minlength` minimum length for summary
    /// * `maxlength` maximum length for summary
    pub async fn summary(&self, text: &str, minlength: Option<i32>, maxlength: Option<i32>) -> Strings {
        // Query parameters
        let mut params = vec![("text", text)];

        let minl = minlength.unwrap_or(-1).to_string();
        let maxl = maxlength.unwrap_or(-1).to_string();

        if minl != "-1" {
            params.push(("minlength", &minl));
        }
        if maxl != "-1" {
            params.push(("maxlength", &maxl));
        }

        // Execute API call
        Ok(self.api.get("summary", &params).await?.json().await?)
    }

    /// Runs a summarization model against a block of text.
    /// 
    /// # Arguments
    /// * `texts` list of text to summarize
    /// * `minlength` minimum length for summary
    /// * `maxlength` maximum length for summary
    pub async fn batchsummary(&self, texts: &Vec<&str>, minlength: Option<i32>, maxlength: Option<i32>) -> StringsBatch {
        // Post parameters
        let params = json!({
            "text": texts,
            "minlength": minlength,
            "maxlength": maxlength
        });

        // Execute API call
        Ok(self.api.post("batchsummary", &params).await?.json().await?)
    }
}
