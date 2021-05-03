use serde_json::json;

pub use crate::api::{API, Texts, TextsBatch};

/// Workflow definition
pub struct Workflow {
    api: API
}

/// Workflow implementation
impl Workflow {
    /// Creates a Workflow instance.
    ///
    /// # Arguments
    /// * `url` - base url of txtai API
    pub fn new(url: &str) -> Workflow {
        Workflow {
            api: API::new(url)
        }
    }

    /// Executes a named workflow using elements as input.
    ///
    /// `name` workflow name
    /// `elements` list of elements to run through workflow
    pub async fn workflow(&self, name: &str, elements: &Vec<&str>) -> TextsBatch {
        // Query parameters
        let params = json!({
            "name": name,
            "elements": elements
        });

        // Execute API call
        Ok(self.api.post("workflow", &params).await?.json().await?)
    }
}
