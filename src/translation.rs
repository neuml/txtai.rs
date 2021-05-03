use serde_json::json;

pub use crate::api::{API, Texts, TextsBatch};

/// Translation definition
pub struct Translation {
    api: API
}

/// Translation implementation
impl Translation {
    /// Creates a Translation instance.
    ///
    /// # Arguments
    /// * `url` - base url of txtai API
    pub fn new(url: &str) -> Translation {
        Translation {
            api: API::new(url)
        }
    }

    /// Translates text from source language into target language.
    ///
    /// # Arguments
    /// * `text` text to translate
    /// * `target` target language code, defaults to "en"
    /// * `source` source language code, detects language if not provided
    pub async fn translate(&self, text: &str, target: Option<&str>, source: Option<&str>) -> Texts {
        // Query parameters
        let mut params = vec![("text", text)];

        let tgt = target.unwrap_or("");
        let src = source.unwrap_or("");

        if tgt != "" {
            params.push(("target", &tgt));
        }
        if src != "" {
            params.push(("source", &src));
        }

        // Execute API call
        Ok(self.api.get("translate", &params).await?.json().await?)
    }

    /// Translates text from source language into target language.
    ///
    /// # Arguments
    /// * `texts` list of text to translate
    /// * `target` target language code, defaults to "en"
    /// * `source` source language code, detects language if not provided
    pub async fn batchtranslate(&self, texts: &Vec<&str>, target: Option<&str>, source: Option<&str>) -> TextsBatch {
        // Post parameters
        let params = json!({
            "text": texts,
            "target": target,
            "source": source
        });

        // Execute API call
        Ok(self.api.post("batchtranslate", &params).await?.json().await?)
    }
}
