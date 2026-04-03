use drift_core::audio::wav::{write_wav_i16, WavSpec};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44_100;
    let duration_seconds = 2.0;
    let total_samples = (sample_rate as f32 * duration_seconds) as usize;

    let frequency = 470.0; // A4
    let sample_rate_f32 = sample_rate as f32;

    let samples: Vec<f32> = (0..total_samples)
        .map(|n| {
            let t = n as f32 / sample_rate_f32;
            (2.0 * std::f32::consts::PI * frequency * t).sin()
        })
        .collect();

    write_wav_i16(
        "out.wav",
        &samples,
        WavSpec {
            sample_rate,
            channels: 1,
        },
    )?;

    println!("Wrote out.wav 🎧");
    Ok(())
}