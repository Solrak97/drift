pub enum Oscillator {
    Sine(SineOscillator),
    Saw(SawOscillator),
    Square(SquareOscillator),
}

impl Oscillator {
    pub fn next_sample(&mut self) -> f32 {
        match self {
            Oscillator::Sine(osc) => osc.next_sample(),
            Oscillator::Saw(osc) => osc.next_sample(),
            Oscillator::Square(osc) => osc.next_sample(),
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        match self {
            Oscillator::Sine(osc) => osc.frequency = frequency,
            Oscillator::Saw(osc) => osc.frequency = frequency,
            Oscillator::Square(osc) => osc.frequency = frequency,
        }
    }
}

pub struct SineOscillator {
    frequency: f32,
    phase: f32,
}

impl SineOscillator {

    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
        }
    }
    
    fn next_sample(&mut self) -> f32 {
        let sample = (2.0 * std::f32::consts::PI * self.frequency * self.phase).sin();
        self.phase += 1.0 / 44100.0; // Assuming a sample rate of 44.1 kHz
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}


pub struct SawOscillator {
    frequency: f32,
    phase: f32,
}

impl SawOscillator {

    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
        }
    }

    fn next_sample(&mut self) -> f32 {
        let sample = 2.0 * (self.phase - (self.phase + 0.5).floor());
        self.phase += 1.0 / 44100.0; // Assuming a sample rate of 44.1 kHz
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

pub struct SquareOscillator {
    frequency: f32,
    phase: f32,
}

impl SquareOscillator {

    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
        }
    }

    fn next_sample(&mut self) -> f32 {
        let sample = if self.phase < 0.5 { 1.0 } else { -1.0 };
        self.phase += 1.0 / 44100.0; // Assuming a sample rate of 44.1 kHz
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}