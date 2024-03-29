mod synths;
mod rack;
mod instruments;

extern crate cpal;
extern crate num_complex;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread::sleep;
use std::time::Duration;
use cpal::{InputCallbackInfo, OutputCallbackInfo};
use crate::instruments::{GPulsePad, MegaDrone};

use crate::synths::{PulseSynth, SawtoothSynth, Synth};

struct AudioCallback {
    drone: MegaDrone,
    drone2: MegaDrone,
    drone3: MegaDrone
}
impl AudioCallback {
    fn new(sample_rate: usize) -> AudioCallback {
        AudioCallback {
            drone: MegaDrone::new(sample_rate, 110f32, 100),
            drone2: MegaDrone::new(sample_rate, 55f32, 100),
            drone3: MegaDrone::new(sample_rate, 220f32, 100)
        }
    }
    fn out_audio_callback(&mut self, data: &mut [f32], _: &cpal::OutputCallbackInfo) {
        let _rng = rand::thread_rng();

        for sample in data.chunks_mut(2) {
            let base = self.drone2.get_sample();
            let s = (self.drone.get_sample() + base) / 2f32;
            let s2= (self.drone3.get_sample() + base) / 2f32;
            //let x: [f32; 2] = [rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)];
            let x: [f32; 2] = [s, s2];
            //let x: [f32, 2] = []
            sample.copy_from_slice(&x);
        }
    }
    fn in_audio_callback(&mut self, _data: &[f32], _: &cpal::InputCallbackInfo) {
        /*for sample in data {
            println!("{:?}", sample);
        }*/
        /*let mut datacp: &mut [f32] = data.clone();
        for sample in datacp.chunks_mut(1) {
            println!("{:?}", sample);
        }*/
    }

}



fn main() {

    let p = PulseSynth::new(440f32, 0.3f32, 0.2f32, 48000);
    p.print_info();
    let host = cpal::default_host();

    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().unwrap().config();
    let sample_rate = config.sample_rate;

    let mut ac_in = AudioCallback::new(sample_rate.0 as usize);
    let _y = move |data: &[f32], info: &InputCallbackInfo| {
        ac_in.in_audio_callback(data, info);
    };


    let mut ac_out = AudioCallback::new(sample_rate.0 as usize);
    let x = move |data: &mut [f32], info: &OutputCallbackInfo| {
        //ac.audio_callback(data, info);
        ac_out.out_audio_callback(data, info);
    };

    /*let in_stream = device.build_input_stream(
        &config,
        y,
        move |err| {
            println!("Error while opening inputstream!");
        },
        None
    ).unwrap();*/
    let out_stream = device.build_output_stream(
        &config,
        x,
        move |_err| {  // react to errors here.

        },
        None // None=blocking, Some(Duration)=timeout
    ).unwrap();




    out_stream.play().expect("Failed to play stream");

    sleep(Duration::from_secs(1000));

    out_stream.pause().expect("Failed to pause stream");
}
