use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

pub use crate::api::{API, APIResponse, IndexResults, IndexResultsBatch};

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
    /// * `query` - query text
    /// * `limit` - maximum results
    pub async fn query(&self, query: &str, limit: i32) -> APIResponse {
        // Query parameters
        let params = [("query", query), ("limit", &limit.to_string())];

        // Execute API call
        Ok(self.api.get("search", &params).await?)
    }

    /// Finds documents in the embeddings model most similar to the input query. Returns
    /// a list of {id: value, score: value} sorted by highest score, where id is the
    /// document id in the embeddings model.
    /// 
    /// # Arguments
    /// * `query` - query text
    /// * `limit` - maximum results
    pub async fn search(&self, query: &str, limit: i32) -> SearchResults {
        // Execute API call and map JSON
        Ok(self.query(query, limit).await?.json().await?)
    }

    /// Finds documents in the embeddings model most similar to the input queries. Returns
    /// a list of {id: value, score: value} sorted by highest score per query, where id is
    /// the document id in the embeddings model.
    ///
    /// # Arguments
    /// * `queries` - queries text
    /// * `limit` - maximum results
    pub async fn batchsearch(&self, queries: &str, limit: i32) -> SearchResultsBatch {
        // Post parameters
        let params = json!({"queries": queries, "limit": limit});

        // Execute API call
        Ok(self.api.post("batchsearch", &params).await?.json().await?)
    }

    /// Adds a batch of documents for indexing.
    /// 
    /// # Arguments
    /// * `documents` - list of {id: value, text: value}
    pub async fn add<T: Serialize>(&self, documents: &Vec<T>) -> APIResponse {
        // Execute API call
        Ok(self.api.post("add", &json!(documents)).await?)
    }

    /// Builds an embeddings index for previously batched documents.
    pub async fn index(&self) -> APIResponse {
        // Execute API call
        Ok(self.api.get("index", &[]).await?)
    }

    /// Runs an embeddings upsert operation for previously batched documents.
    pub async fn upsert(&self) -> APIResponse {
        // Execute API call
        Ok(self.api.get("upsert", &[]).await?)
    }

     /// Deletes from an embeddings index. Returns list of ids deleted.
     ///
     /// # Arguments
     /// * `ids` - list of ids to delete
    pub async fn delete(&self, ids: &Vec<&str>) -> Ids {
        // Execute API call
        Ok(self.api.post("delete", &json!(ids)).await?.json().await?)
    }

    /// Total number of elements in this embeddings index.
    pub async fn count(&self) -> Count {
        Ok(self.api.get("count", &[]).await?.json().await?)
    }

    /// Computes the similarity between query and list of text. Returns a list of
    /// {id: value, score: value} sorted by highest score, where id is the index
    /// in texts.
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
    /// of {id: value, score: value} sorted by highest score per query, where id is the
    /// index in texts.
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

    /// Transforms text into an embeddings array.
    /// 
    /// # Arguments
    /// * `text` - input text
    pub async fn transform(&self, text: &str) -> Embedding {
        // Query parameters
        let params = [("text", text)];

        // Execute API call
        Ok(self.api.get("transform", &params).await?.json().await?)
    }

    /// Transforms list of text into embeddings arrays.
    ///
    /// # Arguments
    /// * `texts` - lists of text
    pub async fn batchtransform(&self, texts: &str) -> EmbeddingBatch {
        // Execute API call
        Ok(self.api.post("batchtransform", &json!(texts)).await?.json().await?)
    }
}

// Embeddings return types
pub type Embedding = Result<Vec<f32>, Box<dyn Error>>;
pub type EmbeddingBatch = Result<Vec<Vec<f32>>, Box<dyn Error>>;
pub type Ids = Result<Vec<String>, Box<dyn Error>>;
pub type Count = Result<usize, Box<dyn Error>>;
pub type SearchResults = Result<Vec<SearchResult>, Box<dyn Error>>;
pub type SearchResultsBatch = Result<Vec<Vec<SearchResult>>, Box<dyn Error>>;

/// Input document
#[derive(Debug, Serialize)]
pub struct Document {
    pub id: String,
    pub text: String
}

// Search result
#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32
}
