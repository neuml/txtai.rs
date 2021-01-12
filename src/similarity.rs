use serde_json::json;

pub use crate::api::{API, IndexResults, IndexResultsBatch};

/// Similarity definition
pub struct Similarity {
    api: API
}

/// Similarity implementation
impl Similarity {
    /// Creates a Similarity instance.
    ///
    /// # Arguments
    /// * `url` - base url of txtai API
    pub fn new(url: &str) -> Similarity {
        Similarity {
            api: API::new(url)
        }
    }

    /// Computes the similarity between query and list of text. Returns a list of
    /// (id, score) sorted by highest score, where id is the index in texts.
    ///
    /// # Arguments
    /// * `query` - query text
    /// * `texts` - list of text
    pub async fn similarity(&self, query: &str, texts: &Vec<&str>) -> IndexResults {
        // Post parameters
        let params = json!({"query": query, "texts": texts});

        // Execute API call
        Ok(self.api.post("similarity", &params).await?.json().await?)
    }

    /// Computes the similarity between list of queries and list of text. Returns a list
    /// of (id, score) sorted by highest score per query, where id is the index in texts.
    ///
    /// # Arguments
    /// * `queries` - queries text
    /// * `texts` - list of text
    pub async fn batchsimilarity(&self, queries: &Vec<&str>, texts: &Vec<&str>) -> IndexResultsBatch {
        // Post parameters
        let params = json!({"queries": queries, "texts": texts});

        // Execute API call
        Ok(self.api.post("batchsimilarity", &params).await?.json().await?)
    }
}
