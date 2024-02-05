use cpal::SampleRate;

pub trait Synth {
    fn get_sample(&mut self) -> f32;
    fn set_frequency(&mut self, freq: f32);
}

pub struct SineSynth {
    phase: f32,
    sample_rate: usize,
    freq: f32,
    samples_per_phase: usize
}

impl SineSynth {
    pub(crate) fn new(sample_rate: usize) -> SineSynth {
        SineSynth {
            phase: 0.0_f32,
            sample_rate,
            freq: 220_f32,
            samples_per_phase: sample_rate/220_f32 as usize
        }
    }
}

impl Synth for SineSynth {

    fn get_sample(&mut self) -> f32 {
        self.phase += (self.freq / self.sample_rate as f32) * 2.0 * std::f32::consts::PI;
        self.phase.sin()
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
            1f32
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

pub struct SawtoothSynth {
    sample: f32,
    sample_rate: usize,
    freq: f32
}

impl Synth for SawtoothSynth {
    fn get_sample(&mut self) -> f32 {
        self.sample += self.freq/self.sample_rate as f32;
        if self.sample > 1f32 {
            self.sample = -1f32;
        }
        self.sample*0.5
    }

    fn set_frequency(&mut self, freq: f32) {
        self.freq = freq;
    }
}

impl SawtoothSynth {
    pub(crate) fn new(sample_rate: usize) -> SawtoothSynth {
        SawtoothSynth {
            sample: 0.0,
            sample_rate,
            freq: 440.0_f32,
        }
    }
}

pub struct PulseSynth {
    phase: usize,
    pulse_pos: f32,
    pulse_size: f32,
    freq: f32,
    sample_rate: usize,
    samples_per_phase: usize,
    pulse_start: usize,
    pulse_end: usize
}

impl Synth for PulseSynth {
    fn get_sample(&mut self) -> f32 {
        self.phase += 1;
        if self.phase >= self.samples_per_phase {
            self.phase = 0;
        }
        if self.phase >= self.pulse_start && self.phase < self.pulse_end {
            return 1 as f32;
        }
        else {
            return 0 as f32;
        }

    }

    fn set_frequency(&mut self, freq: f32) {
        self.freq = freq;
    }
}

impl PulseSynth {
    pub fn new(freq: f32, pulse_pos: f32, pulse_size: f32, sample_rate: usize) -> PulseSynth {
        PulseSynth {
            phase: 0,
            pulse_pos,
            pulse_size,
            freq,
            sample_rate,
            samples_per_phase: sample_rate/freq as usize,
            pulse_start: (pulse_pos * sample_rate as f32/freq) as usize,
            pulse_end: ((pulse_pos * sample_rate as f32/freq) + (pulse_size * sample_rate as f32/freq)) as usize,
        }
    }
    pub fn print_info(&self) {
        println!("samples per phase {}\npulse start {}\npulse end {}\n", self.samples_per_phase, self.pulse_start, self.pulse_end);
    }
}