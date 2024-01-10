pub struct SineSynth {
    phase: f32,
    sample_rate: i32
}

impl SineSynth {
    pub fn new(sample_rate: i32) -> SineSynth {
        SineSynth {
            phase: 0.0_f32,
            sample_rate
        }
    }
    pub fn get_sample(&mut self) -> f32 {
        self.phase += (220.0_f32 / self.sample_rate as f32) * 2.0 * std::f32::consts::PI;
        return self.phase.sin();
    }
}