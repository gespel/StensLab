mod synths;
mod rack;

extern crate cpal;
extern crate num_complex;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread::sleep;
use std::time::Duration;
use cpal::{InputCallbackInfo, OutputCallbackInfo};
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
    fn out_audio_callback(&mut self, data: &mut [f32], _: &cpal::OutputCallbackInfo) {
        let mut rng = rand::thread_rng();
        for sample in data.chunks_mut(2) {
            let s = self.sine.get_sample();
            //let x: [f32; 2] = [rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)];
            let x: [f32; 2] = [s, s];
            //let x: [f32, 2] = []
            sample.copy_from_slice(&x);
        }
    }
    fn in_audio_callback(&mut self, data: &[f32], _: &cpal::InputCallbackInfo) {
        for sample in data {
            println!("{:?}", sample);
        }
        /*let mut datacp: &mut [f32] = data.clone();
        for sample in datacp.chunks_mut(1) {
            println!("{:?}", sample);
        }*/
    }

}



fn main() {
    let host = cpal::default_host();

    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().unwrap().config();
    let sample_rate = config.sample_rate;

    let mut ac_in = AudioCallback::new(sample_rate.0 as usize);
    let y = move |data: &[f32], info: &InputCallbackInfo| {
        ac_in.in_audio_callback(data, info);
    };


    let mut ac_out = AudioCallback::new(sample_rate.0 as usize);
    let x = move |data: &mut [f32], info: &OutputCallbackInfo| {
        //ac.audio_callback(data, info);
        ac_out.out_audio_callback(data, info);
    };

    let in_stream = device.build_input_stream(
        &config,
        y,
        move |err| {

        },
        None
    ).unwrap();
    let out_stream = device.build_output_stream(
        &config,
        x,
        move |err| {  // react to errors here.

        },
        None // None=blocking, Some(Duration)=timeout
    ).unwrap();




    out_stream.play().expect("Failed to play stream");

    sleep(Duration::from_secs(10));

    out_stream.pause().expect("Failed to pause stream");
}
