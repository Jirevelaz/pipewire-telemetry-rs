use pipewire_telemetry_rs::{Engine, AudioEvent};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("--- PipeWire Telemetry Engine (Mock Mode) ---");
    
    let engine = Engine::start_mock().await?;
    let mut rx = engine.subscribe();

    println!("Listening to telemetry stream...");
    
    while let Some(event) = rx.recv().await {
        match event {
            AudioEvent::Rms(level) => {
                let bars = (level * 60.0).max(0.0) as usize;
                let bar_str = "█".repeat(bars);
                print!("\rLevel: [{:<60}] {:.4}", bar_str, level);
            }
        }
    }
    
    Ok(())
}
