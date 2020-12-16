use serde::Serialize;
use serde_json::json;

pub use crate::api::{API, APIResponse, APIScores, APIValues};

/// Embeddings definition
pub struct Embeddings {
    api: API
}

/// Embeddings implementation
impl Embeddings {
    /// Creates an Embeddings instance.
    /// 
    /// # Arguments
    /// * `url` - base url of txtai API
    pub fn new(url: &str) -> Embeddings {
        Embeddings {
            api: API::new(url)
        }
    }

    /// Runs an Embeddings search. Returns Response. This method allows
    /// callers to customize the serialization of the response.
    /// 
    /// # Arguments
    /// * `q` - query string
    /// * `n` - number of results to return
    pub async fn query(&self, q: &str, n: i32) -> APIResponse {
        // Query parameters
        let params = [("q", q), ("n", &n.to_string())];

        // Execute API call
        Ok(self.api.get("search", &params).await?)
    }

    /// Runs an Embeddings search. Returns Vec<Document>. 
    /// 
    /// # Arguments
    /// * `q` - query string
    /// * `n` - number of results to return
    pub async fn search(&self, q: &str, n: i32) -> APIScores {
        // Execute API call and map JSON
        Ok(self.query(q, n).await?.json().await?)
    }

    /// Adds a batch of documents for indexing.
    /// 
    /// # Arguments
    /// * `documents` - list of objects each containing an id and text element
    pub async fn add<T: Serialize>(&self, documents: &Vec<T>) -> APIResponse {
        // Execute API call
        Ok(self.api.post("add", &json!(documents)).await?)
    }

    /// Builds an embeddings index. No further documents can be added after this call.
    pub async fn index(&self) -> APIResponse {
        // Execute API call
        Ok(self.api.get("index", &[]).await?)
    }
    
    /// Calculates the similarity between search and list of elements in data.
    /// 
    /// # Arguments
    /// * `search` - search text
    /// * `data` - list of text to compare against
    pub async fn similarity(&self, search: &str, data: &Vec<&str>) -> APIValues {
        // Post parameters
        let params = json!({"search": search, "data": data});

        // Execute API call
        Ok(self.api.post("similarity", &params).await?.json().await?)
    }

    /// Transforms text into an embeddings array.
    /// 
    /// # Arguments
    /// * `t` - input text
    pub async fn embeddings(&self, t: &str) -> APIScores {
        // Query parameters
        let params = [("t", t)];

        // Execute API call
        Ok(self.api.get("embeddings", &params).await?.json().await?)
    }
}

/// Input document
#[derive(Debug, Serialize)]
pub struct Document {
    pub id: String,
    pub text: String
}
