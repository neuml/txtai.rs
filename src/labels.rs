use serde_json::json;

pub use crate::api::{API, IndexResults, IndexResultsBatch};

/// Labels definition
pub struct Labels {
    api: API
}

/// Labels implementation
impl Labels {
    /// Creates a Labels instance.
    /// 
    pub fn new() -> Labels {
        Labels { 
            api: API::new()
        }
    }

    /// Creates a Labels instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    pub fn with_url(url: &str) -> Labels {
        Labels {
            api: API::with_url(url) 
        }
    }

    /// Creates a Labels instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    /// * `token` - API token
    pub fn with_url_token(url: &str, token: &str) -> Labels {
        Labels {
            api: API::with_url_token(url, token)
        }
    }

    /// Applies a zero shot classifier to text using a list of labels. Returns a list of
    /// {id: value, score: value} sorted by highest score, where id is the index in labels.
    ///
    /// # Arguments
    /// * `text` - input text
    /// * `labels` - list of labels
    pub async fn label(&self, text: &str, labels: &Vec<&str>) -> IndexResults {
        // Post parameters
        let params = json!({"text": text, "labels": labels});

        // Execute API call
        Ok(self.api.post("label", &params).await?.json().await?)
    }

    /// Applies a zero shot classifier to list of text using a list of labels. Returns a list of
    /// {id: value, score: value} sorted by highest score, where id is the index in labels per
    /// text element.
    ///
    /// # Arguments
    /// * `texts` - list of texts
    /// * `labels` - list of labels
    pub async fn batchlabel(&self, texts: &Vec<&str>, labels: &Vec<&str>) -> IndexResultsBatch {
        // Post parameters
        let params = json!({"texts": texts, "labels": labels});

        // Execute API call
        Ok(self.api.post("batchlabel", &params).await?.json().await?)
    }
}
