use serde_json::json;

pub use crate::api::{API, Strings, StringsBatch};

/// Transcription definition
pub struct Transcription {
    api: API
}

/// Transcription implementation
impl Transcription {
    /// Creates a Transcription instance.
    /// 
    pub fn new() -> Transcription {
        Transcription { 
            api: API::new()
        }
    }

    /// Creates an Transcription instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    pub fn with_url(url: &str) -> Transcription {
        Transcription {
            api: API::with_url(url) 
        }
    }

    /// Creates a Transcription instance.
    /// 
    /// # Arguments
    /// * `url` - API url
    /// * `token` - API token
    pub fn with_url_token(url: &str, token: &str) -> Transcription {
        Transcription {
            api: API::with_url_token(url, token)
        }
    }

    /// Transcribes audio files to text.
    ///
    /// `file` file to transcribe
    pub async fn transcribe(&self, file: &str) -> Strings {
        // Query parameters
        let params = [("file", file)];

        // Execute API call
        Ok(self.api.get("transcribe", &params).await?.json().await?)
    }

    /// Transcribes audio files to text.
    ///
    /// `files` list of files to transcribe
    pub async fn batchtranscribe(&self, files: &Vec<&str>) -> StringsBatch {
        // Post parameters
        let params = json!(files);

        // Execute API call
        Ok(self.api.post("batchtranscribe", &params).await?.json().await?)
    }
}
