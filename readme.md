# elevenlabs_sfx

[![Crates.io](https://img.shields.io/crates/v/elevenlabs_sfx.svg)](https://crates.io/crates/elevenlabs_sfx)
[![Docs.rs](https://docs.rs/elevenlabs_sfx/badge.svg)](https://docs.rs/elevenlabs_sfx)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

A type-safe, async Rust client for the [ElevenLabs Sound Effects API](https://elevenlabs.io/app/sound-effects). Generate high-quality sound effects with a simple, ergonomic API.

## Features

- **Type-safe & Async**: Built with Rust's type system and async/await support
- **Builder Pattern**: Intuitive, chainable API for configuring SFX requests
- **Customizable**: Output formats, Duration seconds and Prompt influence
- **Tokio Ready**: Works seamlessly with the Tokio runtime

## Check-out Also:

- **[Elevenlabs TTS](https://github.com/hamzaelmarjani/elevenlabs_tts)**: A type-safe, async Rust client for the ElevenLabs Text-to-Speech API.
- **[Elevenlabs STT](https://github.com/hamzaelmarjani/elevenlabs_stt)**: A type-safe, async Rust client for the ElevenLabs Speech To Text API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs_sfx = "0.0.1"
```

## Quick Start

```rust
use elevenlabs_sfx::ElevenLabsSFXClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ElevenLabsSFXSClient::new("your-api-key");

    let audio = client
        .sound_effects("Ghost Breath Wind")
        .execute()
        .await?;

    std::fs::write("output.mp3", audio)?;
    Ok(())
}
```

## Examples

### Basic Usage

```rust
use elevenlabs_sfx::ElevenLabsSFXClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let api_key = env::var("ELEVENLABS_API_KEY")?;

    let client = ElevenLabsSFXClient::new(api_key);

    let audio = client
        .sound_effects("Orange juice pouring into the glass")
        .execute()
        .await?;

    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &audio)?;
    println!("Audio saved to {}", file_name);

    Ok(())
}
```

### Advanced Configuration

```rust
use elevenlabs_sfx::ElevenLabsSFXClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("ELEVENLABS_API_KEY")?;
    let client = ElevenLabsSFXClient::new(api_key);

     let client = ElevenLabsSFXClient::new(api_key);

    let prompt = "Whoosh whistle flyby air swing swish transition passby bright";

    let audio = client
        .sound_effects(prompt)
        .output_format("mp3_44100_128")
        .duration_seconds(4.5)
        .prompt_influence(0.75)
        .execute()
        .await?;

    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &audio)?;
    println!("Audio saved to {}", file_name);

    Ok(())
}
```

### Running Examples

```bash
# Set your API key
export ELEVENLABS_API_KEY=your_api_key_here

# Run the basic example
cargo run --example basic_sfx

# Run the advanced example
cargo run --example advanced_sfx
```

## API Overview

| Method                             | Description                                      |
| ---------------------------------- | ------------------------------------------------ |
| `ElevenLabsSFXClient::new(String)` | Create client instance (required)\*              |
| `.sound_effects(String)`           | Build a SFX request (required)\*                 |
| `.output_format(String)`           | Audio format (e.g. mp3_44100) (optional)         |
| `.duration_seconds(f32)`           | Duration in seconds (0.5 - 22.0) (optional)      |
| `.prompt_influence(f32)`           | Prompt adherence strength (0.0 - 1.0) (optional) |
| `.execute()`                       | Run request → audio (required)\*                 |

## Error Handling

The crate uses standard Rust error handling patterns. All async methods return `Result` types:

```rust
match client.sound_effects("Ghost Breath Wind").execute().await {
    Ok(audio) => println!("Generated {} bytes of audio", audio.len()),
    Err(e) => eprintln!("SFX generation failed: {}", e),
}
```

## Requirements

- Rust 1.70+ (for async/await support)
- Tokio runtime
- Valid ElevenLabs API key

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

Contributions are welcome! Please feel free to:

- Open issues for bugs or feature requests
- Submit pull requests with improvements
- Improve documentation or examples
- Add tests or benchmarks

Before contributing, please ensure your code follows Rust conventions and includes appropriate tests.

## Support

If you like this project, consider supporting me on Patreon 💖

[![Patreon](https://img.shields.io/badge/Support-Patreon-orange.svg)](https://www.patreon.com/elmarjanihamza/gift)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Note**: This crate is not officially affiliated with ElevenLabs. Please refer to the [ElevenLabs API documentation](https://elevenlabs.io/docs/api-reference/text-to-sound-effects/convert) for the most up-to-date API information.
