use reqwest::multipart;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
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
    pub fn new() -> Embeddings {
        Embeddings { 
            api: API::new()
        }
    }

    /// Creates an Embeddings instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    pub fn with_url(url: &str) -> Embeddings {
        Embeddings {
            api: API::with_url(url) 
        }
    }

    /// Creates an Embeddings instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    /// * `token` - API token
    pub fn with_url_token(url: &str, token: &str) -> Embeddings {
        Embeddings {
            api: API::with_url_token(url, token)
        }
    }

    /// Runs an Embeddings search. Returns Response. This method allows
    /// callers to customize the serialization of the response.
    /// 
    /// # Arguments
    /// * `query` - query text
    /// * `limit` - maximum results
    /// * `weights` - hybrid score weights, if applicable
    /// * `index` - index name, if applicable
    pub async fn query(&self, query: &str, limit: i32, weights: Option<f32>, index: Option<&str>) -> APIResponse {
        // Query parameters
        let mut params = vec![("query", query)];

        let limitl = limit.to_string();
        let weightsl = weights.unwrap_or(-1.0).to_string();
        let indexl = index.unwrap_or("").to_string();

        if limitl != "" {
            params.push(("limit", &limitl));
        }
        if weightsl != "-1.0" {
            params.push(("weights", &weightsl));
        }
        if indexl != "" {
            params.push(("index", &indexl));
        }

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
    /// * `weights` - hybrid score weights, if applicable
    /// * `index` - index name, if applicable
    pub async fn search(&self, query: &str, limit: i32, weights: Option<f32>, index: Option<&str>) -> SearchResults {
        // Execute API call and map JSON
        Ok(self.query(query, limit, weights, index).await?.json().await?)
    }

    /// Finds documents in the embeddings model most similar to the input queries. Returns
    /// a list of {id: value, score: value} sorted by highest score per query, where id is
    /// the document id in the embeddings model.
    ///
    /// # Arguments
    /// * `queries` - queries text
    /// * `limit` - maximum results
    /// * `weights` - hybrid score weights, if applicable
    /// * `index` - index name, if applicable
    pub async fn batchsearch(&self, queries: &Vec<&str>, limit: i32, weights: Option<f32>, index: Option<&str>) -> SearchResultsBatch {
        // Post parameters
        let params = json!({
            "queries": queries,
            "limit": limit,
            "weights": weights,
            "index": index
        });

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

    /// Recreates this embeddings index using config. This method only works if document content storage is enabled.
    pub async fn reindex(&self, config: HashMap<&str, &str>, function: Option<&str>) -> APIResponse {
        // Post parameters
        let params = json!({
            "config": config,
            "function": function
        });

        // Execute API call
        Ok(self.api.post("reindex", &params).await?)
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

    /// Adds a batch of binary objects for indexing.
    ///
    /// # Arguments
    /// * `data` - list of binary data
    /// * `uid` - list of corresponding ids (optional)
    /// * `field` - optional object field name
    pub async fn addobject(&self, data: Vec<Vec<u8>>, uid: Option<Vec<&str>>, field: Option<&str>) -> APIResponse {
        let mut form = multipart::Form::new();

        // Add binary data
        for (i, bytes) in data.into_iter().enumerate() {
            let part = multipart::Part::bytes(bytes)
                .file_name(format!("file{}", i))
                .mime_str("application/octet-stream")?;
            form = form.part("data", part);
        }

        // Add uid values
        if let Some(ids) = uid {
            for id in ids {
                form = form.text("uid", id.to_string());
            }
        }

        // Add field
        if let Some(f) = field {
            form = form.text("field", f.to_string());
        }

        // Execute API call
        Ok(self.api.post_multipart("addobject", form).await?)
    }

    /// Adds a batch of images for indexing.
    ///
    /// # Arguments
    /// * `data` - list of image data
    /// * `uid` - list of corresponding ids
    /// * `field` - optional object field name
    pub async fn addimage(&self, data: Vec<Vec<u8>>, uid: Option<Vec<&str>>, field: Option<&str>) -> APIResponse {
        let mut form = multipart::Form::new();

        // Add image data
        for (i, bytes) in data.into_iter().enumerate() {
            let part = multipart::Part::bytes(bytes)
                .file_name(format!("image{}.jpg", i))
                .mime_str("image/jpeg")?;
            form = form.part("data", part);
        }

        // Add uid values
        if let Some(ids) = uid {
            for id in ids {
                form = form.text("uid", id.to_string());
            }
        }

        // Add field
        if let Some(f) = field {
            form = form.text("field", f.to_string());
        }

        // Execute API call
        Ok(self.api.post_multipart("addimage", form).await?)
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
