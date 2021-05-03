use serde_json::json;

pub use crate::api::{API, Texts, TextsBatch};

/// Summary definition
pub struct Summary {
    api: API
}

/// Summary implementation
impl Summary {
    /// Creates a Summary instance.
    ///
    /// # Arguments
    /// * `url` - base url of txtai API
    pub fn new(url: &str) -> Summary {
        Summary {
            api: API::new(url)
        }
    }

    /// Runs a summarization model against a block of text.
    /// 
    /// # Arguments
    /// * `text` text to summarize
    /// * `minlength` minimum length for summary
    /// * `maxlength` maximum length for summary
    pub async fn summary(&self, text: &str, minlength: Option<i32>, maxlength: Option<i32>) -> Texts {
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
    pub async fn batchsummary(&self, texts: &Vec<&str>, minlength: Option<i32>, maxlength: Option<i32>) -> TextsBatch {
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
