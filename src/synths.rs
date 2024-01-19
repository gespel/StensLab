pub trait Synth {
    fn get_sample(&mut self) -> f32;
    fn set_frequency(&mut self, freq: f32);
}

pub struct SineSynth {
    phase: f32,
    sample_rate: usize,
    freq: f32
}

impl SineSynth {
    pub(crate) fn new(sample_rate: usize) -> SineSynth {
        SineSynth {
            phase: 0.0_f32,
            sample_rate,
            freq: 220_f32
        }
    }
}

impl Synth for SineSynth {

    fn get_sample(&mut self) -> f32 {
        self.phase += (self.freq / self.sample_rate as f32) * 2.0 * std::f32::consts::PI;
        return self.phase.sin();
    }

    fn set_frequency(&mut self, freq: f32) {
        self.freq = freq;
    }
}

pub struct SquareSynth {
    phase: f32,
    sample_rate: usize,
    freq: f32
}

impl SquareSynth {
    fn new(sample_rate: usize) -> Self {
        SquareSynth {
            phase: 0.0,
            sample_rate,
            freq: 220_f32,
        }
    }
}

impl Synth for SquareSynth {
    fn get_sample(&mut self) -> f32 {
        self.phase += (self.freq / self.sample_rate as f32) * 2.0 * std::f32::consts::PI;
        let x = self.phase.sin();
        if x > 0f32 {
            return 1f32;
        }
        else if x == 0f32 {
            return 0f32;
        }
        else {
            return -1f32;
        }
    }

    fn set_frequency(&mut self, freq: f32) {
        self.freq = freq;
    }
}