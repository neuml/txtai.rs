use enum_as_inner::EnumAsInner;
use reqwest::{multipart, RequestBuilder, Response};
use serde::Deserialize;
use serde_json::Value;
use std::env;
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
    url: String,
    token: String
}

/// Base API implementation
impl API {
    /// Creates an API instance.
    ///
    pub fn new() -> API {
        API { 
            url: env::var("TXTAI_API_URL").unwrap_or(String::from("")),
            token: env::var("TXTAI_API_TOKEN").unwrap_or(String::from(""))
        }
    }

    /// Creates an API instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    pub fn with_url(url: &str) -> API {
        API { 
            url: url.to_string(),
            token: env::var("TXTAI_API_TOKEN").unwrap_or(String::from(""))
        }
    }

    /// Creates an API instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    /// * `token` - API token
    pub fn with_url_token(url: &str, token: &str) -> API {
        API { 
            url: url.to_string(),
            token: token.to_string()
        }
    }

    /// Executes a GET request. Returns Response.
    /// 
    /// # Arguments
    /// * `method` - API method
    /// * `params` - List of (name, value) tuples to use as query parameters
    pub async fn get(&self, method: &str, params: &[(&str, &str)]) -> APIResponse {
        // Generate url
        let url = format!("{url}/{method}", url=self.url, method=method);

        // Create client
        let client = reqwest::Client::new();
        let mut request = client.get(&url);

        // Set headers
        request = self.headers(request);

        // Execute API call
        Ok(request.query(&params).send().await?)
    }

    /// Executes a POST request. Returns Response.
    /// 
    /// # Arguments
    /// * `method` - API method
    /// * `json` - Value object with post body 
    pub async fn post(&self, method: &str, json: &Value) -> APIResponse {
        // Generate url
        let url = format!("{url}/{method}", url=self.url, method=method);

        // Create client
        let client = reqwest::Client::new();
        let mut request = client.post(&url);

        // Set headers
        request = self.headers(request);

        // Execute API call
        Ok(request.json(&json).send().await?)
    }

    /// Sets headers on a request.
    ///
    /// # Arguments
    //  * `request` - RequestBuilder
    pub fn headers(&self, request: RequestBuilder) -> RequestBuilder {
        // Authorization header
        if self.token != "" {
            return request.header("Authorization", format!("Bearer {token}", token=self.token))
        }
        return request
    }

    /// Executes a multipart POST request. Returns Response.
    ///
    /// # Arguments
    /// * `method` - API method
    /// * `form` - Multipart form data
    pub async fn post_multipart(&self, method: &str, form: multipart::Form) -> APIResponse {
        // Generate url
        let url = format!("{url}/{method}", url=self.url, method=method);

        // Create client
        let client = reqwest::Client::new();
        let mut request = client.post(&url);

        // Set headers
        request = self.headers(request);

        // Execute API call
        Ok(request.multipart(form).send().await?)
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
