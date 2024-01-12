use crate::synths::Synth;

pub struct Rack {
    osc: Vec<Box<dyn Synth>>
}

impl Rack {
    fn new() -> Rack {
        Rack {
            osc: Vec::new()
        }
    }
    pub fn add_synth<T: Synth + 'static>(&mut self, synth: T) {
        self.osc.push(Box::new(synth));
    }
}