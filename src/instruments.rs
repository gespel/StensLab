use crate::synths::{PulseSynth, Synth};

pub struct GPulsePad {
    p1: PulseSynth,
    p2: PulseSynth,
    p3: PulseSynth
}

impl GPulsePad {
    pub fn new(sample_rate: usize) -> GPulsePad {
        GPulsePad {
            p1: PulseSynth::new(220.4f32, 0.2f32, 0.3f32, sample_rate),
            p2: PulseSynth::new(109.8f32, 0.1f32, 0.3f32, sample_rate),
            p3: PulseSynth::new(55f32, 0.0f32, 0.6f32, sample_rate),
        }
    }

    pub fn get_sample(&mut self) -> f32 {
        let mut s = self.p1.get_sample() + self.p2.get_sample() + self.p3.get_sample();
        s / 3f32
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.p1.set_frequency(freq-0.4f32);
        self.p2.set_frequency((freq/2f32) + 0.4f32);
        self.p3.set_frequency(freq/3f32);
        //a
    }
}