use crate::synth_core::*;
use crate::audio_o::*;

pub(crate) struct M1 {
    t: f32,
    ins: Vec<Port>,
    outs: Vec<Port>,

}

impl Module for M1 {
    fn process(&mut self) {
        self.t += 1.0;
        self.outs[0].value = self.ins[0].value + self.t;
        println!("mod1 t {}", self.t);
    }
    fn inputs(&mut self) -> &mut Vec<Port> {
        &mut self.ins
    }
    fn outputs(&mut self) -> &mut Vec<Port> {
        &mut self.outs
    }
}

impl Default for M1 {
    fn default() -> Self {
        M1 {
            t: 0.0,
            ins: vec![Port { value: 0.0 }],
            outs: vec![Port { value: 0.0 }],
        }
    }
}

pub struct M2 {
    t: f32,
    ins: Vec<Port>,
    outs: Vec<Port>,
}

impl Default for M2 {
    fn default() -> Self {
        M2 {
            t: 0.0,
            ins: vec![Port { value: 0.0 }],
            outs: vec![Port { value: 0.0 }],
        }
    }
}

impl Module for M2 {
    fn process(&mut self) {
        self.t += 23.0;
        self.outs[0].value = self.ins[0].value + self.t;
        println!("mod2 t {}", self.t);
    }
    fn inputs(&mut self) -> &mut Vec<Port> {
        &mut self.ins
    }
    fn outputs(&mut self) -> &mut Vec<Port> {
        &mut self.outs
    }
}


