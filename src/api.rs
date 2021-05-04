use enum_as_inner::EnumAsInner;
use reqwest::Response;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

// Generic return types
pub type APIResponse = Result<Response, Box<dyn Error>>;
pub type IndexResults = Result<Vec<IndexResult>, Box<dyn Error>>;
pub type IndexResultsBatch = Result<Vec<Vec<IndexResult>>, Box<dyn Error>>;
pub type Strings = Result<String, Box<dyn Error>>;
pub type StringsBatch = Result<Vec<String>, Box<dyn Error>>;
pub type Texts = Result<Text, Box<dyn Error>>;
pub type TextsBatch = Result<Vec<Text>, Box<dyn Error>>;

/// Base API definition
pub struct API {
    url: String
}

/// Base API implementation
impl API {
    /// Creates an API instance.
    /// 
    /// # Arguments
    /// * `url` - base url of txtai API
    pub fn new(url: &str) -> API {
        API { url: url.to_string()}
    }

    /// Executes a GET request. Returns Response.
    /// 
    /// # Arguments
    /// * `method` - API method
    /// * `params` - List of (name, value) tuples to use as query parameters
    pub async fn get(&self, method: &str, params: &[(&str, &str)]) -> APIResponse {
        // Generate url
        let url = format!("{url}/{method}", url=self.url, method=method);

        // Create client and add query params
        let client = reqwest::Client::new();
        let request = client.get(&url).query(&params);

        // Execute API call
        Ok(request.send().await?)
    }

    /// Executes a POST request. Returns Response.
    /// 
    /// # Arguments
    /// * `method` - API method
    /// * `json` - Value object with post body 
    pub async fn post(&self, method: &str, json: &Value) -> APIResponse {
        // Generate url
        let url = format!("{url}/{method}", url=self.url, method=method);

        // Create client and add json
        let client = reqwest::Client::new();
        let request = client.post(&url).json(&json);

        // Execute API call
        Ok(request.send().await?)
    }
}

// Index result
#[derive(Debug, Deserialize)]
pub struct IndexResult {
    pub id: usize,
    pub score: f32
}

// Text result that handles String and Vector of Strings
#[derive(Debug, Deserialize, EnumAsInner)]
#[serde(untagged)]
pub enum Text {
    String(String),
    List(Vec<String>)
}
