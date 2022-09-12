use crate::synth_core::*;
use crate::audio_o::*;
struct M1 {
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

struct M2 {
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


pub fn test_engine() -> Engine {
    let mut mods: Vec<Mutex<Arc<dyn Module>>> = vec![Mutex::new(Arc::new(M1::default())),
                                                     Mutex::new(Arc::new(ModuleO::default()))];

    Engine {
        handle: None,
        alive: Arc::new(Default::default()),
        core: Arc::new(std::sync::Mutex::new(RealTimeCore {
            modules_pointers: vec![getPointer(&mut mods, 0), getPointer(&mut mods, 1)],
            cable_core: vec![],
        })),
        cables: vec![Mutex::new(Cable {
            input_module_p: getPointer(&mut mods, 0),
            output_module_p: getPointer(&mut mods, 1),
            input_port: 0,
            output_port: 0,
        })],
        modules: mods,
    }
}

