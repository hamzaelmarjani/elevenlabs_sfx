use serde::Serialize;

/// Request body for sound effects API calls
#[derive(Debug, Clone, Serialize)]
pub struct SFXRequest {
    pub text: String,
    // Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32.
    // MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above.
    // Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    // Possible values are: mp3_22050_32 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96
    // Default to: mp3_44100_128
    // This goes in the URL path, not in the body.
    pub output_format: Option<String>,

    /// The duration of the sound which will be generated in seconds. Must be at least 0.5 and at most 22.
    /// If set to None we will guess the optimal duration using the prompt. Defaults to None.
    pub duration_seconds: Option<f32>,

    /// A higher prompt influence makes your generation follow the prompt more closely while also making generations less variable.
    /// Must be a value between 0 and 1. Defaults to 0.3
    pub prompt_influence: Option<f32>,
}
