use drift_core::audio::wav::{WavSpec, write_wav_i16};
use drift_core::dsp::oscillator::{Oscillator, SawOscillator, SineOscillator, SquareOscillator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44_100;
    let duration_seconds = 5.0;
    let total_samples = (sample_rate as f32 * duration_seconds) as usize;

    let mut oscillator = Oscillator::Saw(SawOscillator::new(440.0));

    let sample_rate_f32 = sample_rate as f32;

    let samples: Vec<f32> = (0..total_samples)
        .map(|n| {
            let t = n as f32 / sample_rate_f32;
            oscillator.next_sample()
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
