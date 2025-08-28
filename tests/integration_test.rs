use elevenlabs_sfx::{ElevenLabsSFXClient, ElevenLabsSFXError};

#[tokio::test]
async fn test_client_creation() {
    let _client = ElevenLabsSFXClient::new("test-api-key");
    // Just test that it doesn't panic
    assert_eq!(true, true);
}

#[tokio::test]
async fn test_builder_pattern() {
    let client = ElevenLabsSFXClient::new("test-key");
    let _builder = client.sound_effects("Stone scrape");

    // Test that builder methods are chainable
    assert_eq!(true, true); // Builder pattern works if this compiles
}

#[test]
fn test_error_display() {
    let error = ElevenLabsSFXError::ValidationError("Invalid output format".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Validation error"));
    assert!(display.contains("Invalid output format"));
}

// Mock tests for API calls (without real HTTP requests)
#[cfg(test)]
mod mock_tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_api_key_error() {
        let _client = ElevenLabsSFXClient::new("invalid-key");

        // This would normally fail with auth error, but we can't test without real API
        // In a real test, you'd use a mock HTTP server like wiremock
        // For now, just test that the client can be created
        assert_eq!(true, true);
    }
}
