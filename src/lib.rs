//! ElevenLabs Sound Effects API client
//!
//! A type-safe, async Rust client for the ElevenLabs SFX API.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use elevenlabs_sfx::ElevenLabsSFXClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ElevenLabsSFXClient::new("your-api-key");
//!     
//!     let audio = client
//!         .sound_effects("Ghost Breath Wind")
//!         .output_format("mp3_44100_128")
//!         .duration_seconds(4.5)
//!         .prompt_influence(0.6)
//!         .execute()
//!         .await?;
//!     
//!     // audio is Vec<u8> - raw audio data
//!     std::fs::write("output.mp3", audio)?;
//!     Ok(())
//! }
//! ```

use reqwest::Client;

pub mod error;
pub mod types;

pub use error::ElevenLabsSFXError;
pub use types::*;

/// Main client for interacting with ElevenLabs API
#[derive(Clone)]
pub struct ElevenLabsSFXClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ElevenLabsSFXClient {
    /// Create a new ElevenLabs client with API key
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: "https://api.elevenlabs.io/v1".to_string(),
        }
    }

    /// Create a new client with custom base URL (for testing/enterprise)
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        }
    }

    /// Start building a sound effects request
    pub fn sound_effects<S: Into<String>>(&self, text: S) -> SoundEffectsBuilder {
        SoundEffectsBuilder::new(self.clone(), text.into())
    }

    /// Internal method to execute SFX request
    pub(crate) async fn execute_sfx(
        &self,
        request: SFXRequest,
    ) -> Result<Vec<u8>, ElevenLabsSFXError> {
        let url = format!("{}/sound-generation", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ElevenLabsSFXError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.bytes().await?.to_vec())
    }
}

/// Builder for sound effects requests
pub struct SoundEffectsBuilder {
    client: ElevenLabsSFXClient,
    text: String,
    output_format: Option<String>,
    duration_seconds: Option<f32>,
    prompt_influence: Option<f32>,
}

impl SoundEffectsBuilder {
    fn new(client: ElevenLabsSFXClient, text: String) -> Self {
        Self {
            client,
            text,
            output_format: None,
            duration_seconds: None,
            prompt_influence: None,
        }
    }

    /// Set the output format to use
    pub fn output_format<S: Into<String>>(mut self, output_format: S) -> Self {
        self.output_format = Some(output_format.into());
        self
    }

    /// Set duration seconds to use
    pub fn duration_seconds(mut self, duration_seconds: f32) -> Self {
        self.duration_seconds = Some(duration_seconds);
        self
    }

    /// Set duration seconds to use
    pub fn prompt_influence(mut self, prompt_influence: f32) -> Self {
        self.prompt_influence = Some(prompt_influence);
        self
    }

    /// Execute the sound effects request
    pub async fn execute(self) -> Result<Vec<u8>, ElevenLabsSFXError> {
        let output_format = self
            .output_format
            .unwrap_or_else(|| "mp3_44100_128".to_string()); // Default to: mp3_44100_128

        let request = SFXRequest {
            text: self.text,
            output_format: Some(output_format.clone()),
            duration_seconds: self.duration_seconds.or(None),
            prompt_influence: self.prompt_influence.or(None),
        };

        self.client.execute_sfx(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = ElevenLabsSFXClient::new("test-key");
        assert_eq!(client.api_key, "test-key");
    }

    #[test]
    fn test_builder_pattern() {
        let client = ElevenLabsSFXClient::new("test-key");
        let builder = client
            .sound_effects("TV-VHS glitch transition")
            .output_format("mp3_44100_128")
            .duration_seconds(3.0)
            .prompt_influence(0.6);

        // Builder pattern works
        assert_eq!(builder.text, "TV-VHS glitch transition");
        assert_eq!(builder.output_format, "mp3_44100_128".to_string().into());
        assert_eq!(builder.duration_seconds, Some(3.0));
        assert_eq!(builder.prompt_influence, Some(0.6));
    }
}
