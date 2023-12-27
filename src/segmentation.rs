use serde_json::json;

pub use crate::api::{API, Texts, TextsBatch};

/// Segmentation definition
pub struct Segmentation {
    api: API
}

/// Segmentation implementation
impl Segmentation {
    /// Creates a Segmentation instance.
    /// 
    pub fn new() -> Segmentation {
        Segmentation { 
            api: API::new()
        }
    }

    /// Creates a Segmentation instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    pub fn with_url(url: &str) -> Segmentation {
        Segmentation {
            api: API::with_url(url) 
        }
    }

    /// Creates a Segmentation instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    /// * `token` - API token
    pub fn with_url_token(url: &str, token: &str) -> Segmentation {
        Segmentation {
            api: API::with_url_token(url, token)
        }
    }

    /// Segments text into semantic units.
    /// 
    /// # Arguments
    /// * `text` input text
    pub async fn segment(&self, text: &str) -> Texts {
        // Query parameters
        let params = [("text", text)];

        // Execute API call
        Ok(self.api.get("segment", &params).await?.json().await?)
    }

    /// Segments text into semantic units.
    /// 
    /// # Arguments
    /// * `texts` list of texts to segment
    pub async fn batchsegment(&self, texts: &Vec<&str>) -> TextsBatch {
        // Post parameters
        let params = json!(texts);

        // Execute API call
        Ok(self.api.post("batchsegment", &params).await?.json().await?)
    }
}
