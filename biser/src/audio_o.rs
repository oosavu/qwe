use crate::synth_core::*;


pub struct ModuleO {
    ins: Vec<Port>,
    outs: Vec<Port>,
}


impl Module for ModuleO {
    fn process(&mut self) {
        println!("modOOOO");
    }
    fn inputs(&mut self) -> &mut Vec<Port> {
        &mut self.ins
    }
    fn outputs(&mut self) -> &mut Vec<Port> {
        &mut self.outs
    }
}

impl Default for ModuleO {
    fn default() -> Self {
        ModuleO {
            ins: vec![Port { value: 0.0 }],
            outs: vec![],
        }
    }
}