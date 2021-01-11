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
    /// # Arguments
    /// * `url` - base url of txtai API
    pub fn new(url: &str) -> Labels {
        Labels {
            api: API::new(url)
        }
    }

    /// Applies a zero shot classifier to text using a list of labels. Returns a list of
    /// (id, score) sorted by highest score, where id is the index in labels.
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
    /// (id, score) sorted by highest score, where id is the index in labels.
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
