use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct WavSpec {
    pub sample_rate: u32,
    pub channels: u16,
}

impl Default for WavSpec {
    fn default() -> Self {
        Self {
            sample_rate: 44_100,
            channels: 1,
        }
    }
}

pub fn write_wav_f32<P: AsRef<Path>>(
    path: P,
    samples: &[f32],
    spec: WavSpec,
) -> Result<(), hound::Error> {
    let wav_spec = hound::WavSpec {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(path, wav_spec)?;

    for &sample in samples {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;
    Ok(())
}

pub fn write_wav_i16<P: AsRef<Path>>(
    path: P,
    samples: &[f32],
    spec: WavSpec,
) -> Result<(), hound::Error> {
    let wav_spec = hound::WavSpec {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, wav_spec)?;

    for &sample in samples {
        let clamped = sample.clamp(-1.0, 1.0);
        let quantized = (clamped * i16::MAX as f32) as i16;
        writer.write_sample(quantized)?;
    }

    writer.finalize()?;
    Ok(())
}

pub fn read_wav_f32<P: AsRef<Path>>(path: P) -> Result<(Vec<f32>, WavSpec), hound::Error> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();

    let drift_spec = WavSpec {
        sample_rate: spec.sample_rate,
        channels: spec.channels,
    };

    let samples = match (spec.sample_format, spec.bits_per_sample) {
        (hound::SampleFormat::Float, 32) => {
            let mut out = Vec::new();
            for sample in reader.samples::<f32>() {
                out.push(sample?);
            }
            out
        }
        (hound::SampleFormat::Int, 16) => {
            let mut out = Vec::new();
            for sample in reader.samples::<i16>() {
                let s = sample? as f32 / i16::MAX as f32;
                out.push(s);
            }
            out
        }
        _ => {
            return Err(hound::Error::FormatError("unsupported WAV format"));
        }
    };

    Ok((samples, drift_spec))
}