use serde_json::json;

pub use crate::api::{API, APIScores};

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

     /// Applies a zero shot classifier to a text section using a list of labels.
     /// 
     /// # Arguments
     /// * text - input text
     /// * labels - list of labels
    pub async fn label(&self, text: &str, labels: &Vec<&str>) -> APIScores {
        // Post parameters
        let params = json!({"text": text, "labels": labels});

        // Execute API call
        Ok(self.api.post("label", &params).await?.json().await?)
    }
}
