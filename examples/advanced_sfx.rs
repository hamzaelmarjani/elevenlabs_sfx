use elevenlabs_sfx::ElevenLabsSFXClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

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
