use crate::synth_core::*;
use crate::audio_o::*;
use crate::*;

pub(crate) struct M1 {
    t: f32,
    ins: Vec<AudioPort>,
    outs: Vec<AudioPort>,

}

impl Module for M1 {
    fn process(&mut self) {
        self.t += 1.0;
        self.outs[0].value[0] = self.ins[0].value[0] + self.t;
        println!("mod1 t {}", self.t);
    }
    fn inputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.ins
    }
    fn outputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.outs
    }
}

impl Default for M1 {
    fn default() -> Self {
        M1 {
            t: 0.0,
            ins: AudioPort::create_audio_ports(1),
            outs: AudioPort::create_audio_ports(1),
        }
    }
}

pub struct M2 {
    t: f32,
    ins: Vec<AudioPort>,
    outs: Vec<AudioPort>,
}

impl Default for M2 {
    fn default() -> Self {
        M2 {
            t: 0.0,
            ins: AudioPort::create_audio_ports(1),
            outs: AudioPort::create_audio_ports(1),
        }
    }
}

impl Module for M2 {
    fn process(&mut self) {
        self.t += 23.0;
        self.outs[0].value[0] = self.ins[0].value[0] + self.t;
        println!("mod2 t {}", self.t);
    }
    fn inputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.ins
    }
    fn outputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.outs
    }
}


