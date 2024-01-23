mod synths;
mod rack;
mod script_language;

extern crate cpal;
extern crate num_complex;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread::sleep;
use std::time::Duration;
use cpal::{InputCallbackInfo, OutputCallbackInfo};
use crate::script_language::ScriptParser;


use crate::synths::{SawtoothSynth, Synth};

struct AudioCallback {
    sine: SawtoothSynth
}
impl AudioCallback {
    fn new(sample_rate: usize) -> AudioCallback {
        AudioCallback {
            sine: SawtoothSynth::new(sample_rate)
        }
    }
    fn out_audio_callback(&mut self, data: &mut [f32], _: &cpal::OutputCallbackInfo) {
        let _rng = rand::thread_rng();
        self.sine.set_frequency(440_f32);
        for sample in data.chunks_mut(2) {
            let s = self.sine.get_sample();
            //let x: [f32; 2] = [rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)];
            let x: [f32; 2] = [s, s];
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
    let sp: ScriptParser = ScriptParser::new(".".to_string());
    sp.print_files();
    sp.parse_files();

    /*let host = cpal::default_host();

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

    sleep(Duration::from_secs(10));

    out_stream.pause().expect("Failed to pause stream");*/
}
