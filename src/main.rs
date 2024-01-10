mod synths;

extern crate cpal;
extern crate num_complex;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use crate::synths::SineSynth;

fn audio_callback(data: &mut [f32], _: &cpal::OutputCallbackInfo) {
    let mut rng = rand::thread_rng();
    for sample in data.chunks_mut(2) {
        let x: [f32; 2] = [rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)];
        //let x: [f32, 2] = []
        sample.copy_from_slice(&x);
    }
}

fn main() {
    let host = cpal::default_host();

    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().unwrap().config();
    let sample_rate = config.sample_rate;
    /*let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut rng = rand::thread_rng();
            for sample in data.chunks_mut(2) {
                let x: [f32; 2] = [rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)];
                sample.copy_from_slice(&x);
            }
        },
        move |err| {
            // react to errors here.
        },
        None // None=blocking, Some(Duration)=timeout
    ).unwrap();*/

    let stream = device.build_output_stream(
        &config,
        audio_callback,
        move |err| {  // react to errors here.

        },
        None // None=blocking, Some(Duration)=timeout
    ).unwrap();

    stream.play().expect("Failed to play stream");

    sleep(Duration::from_secs(1));

    stream.pause().expect("Failed to pause stream");
}
