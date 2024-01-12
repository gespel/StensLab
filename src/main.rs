mod synths;
mod rack;

extern crate cpal;
extern crate num_complex;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread::sleep;
use std::time::Duration;
use cpal::OutputCallbackInfo;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::synths::{SineSynth, Synth};

struct AudioCallback {
    sine: SineSynth
}
impl AudioCallback {
    fn new(sample_rate: usize) -> AudioCallback {
        AudioCallback {
            sine: SineSynth::new(sample_rate as usize)
        }
    }
    fn audio_callback(&mut self, data: &mut [f32], _: &cpal::OutputCallbackInfo) {
        let mut rng = rand::thread_rng();
        for sample in data.chunks_mut(2) {
            let s = self.sine.get_sample();
            //let x: [f32; 2] = [rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)];
            let x: [f32; 2] = [s, s];
            //let x: [f32, 2] = []
            sample.copy_from_slice(&x);
        }
    }
}



fn main() {
    let host = cpal::default_host();

    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().unwrap().config();
    let sample_rate = config.sample_rate;

    let mut ac = AudioCallback::new(sample_rate.0 as usize);
    let x = move |data: &mut [f32], info: &OutputCallbackInfo| {
        //ac.audio_callback(data, info);
        ac.audio_callback(data, info);
    };
    let stream = device.build_output_stream(
        &config,
        x,
        move |err| {  // react to errors here.

        },
        None // None=blocking, Some(Duration)=timeout
    ).unwrap();

    stream.play().expect("Failed to play stream");

    sleep(Duration::from_secs(10));

    stream.pause().expect("Failed to pause stream");
}
