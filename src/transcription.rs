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
    /// # Arguments
    /// * `url` - base url of txtai API
    pub fn new(url: &str) -> Transcription {
        Transcription {
            api: API::new(url)
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
