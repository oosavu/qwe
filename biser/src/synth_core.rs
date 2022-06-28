use std::rc::Rc;
use std::cell::RefCell;

const CHANELS: usize = 16;

#[derive(Clone, Copy)]
struct Port {
    value: f32,
}

trait Module {
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<Port>;
    fn outputs(&mut self) -> &mut Vec<Port>;
}

struct Cable {
    input_module: Rc<RefCell<dyn Module>>,
    output_module: Rc<RefCell<dyn Module>>,
    input_port: usize,
    output_port: usize
}

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

impl Default for M1{
    fn default() -> Self {
        M1{
            t: 0.0,
            ins: vec![Port{ value: 0.0 }],
            outs: vec![Port{ value: 0.0 }],
        }
    }
}

struct M2 {
    t: f32,
    ins: Vec<Port>,
    outs: Vec<Port>,
}

impl Default for M2{
    fn default() -> Self {
        M2{
            t: 0.0,
            ins: vec![Port{ value: 0.0 }],
            outs: vec![Port{ value: 0.0 }],
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


pub struct Engine {
    modules: Vec<Rc<RefCell<dyn Module>>>,
    cables: Vec<Cable>,
}
// fn print_type_of<T>(_: &T) {
//     println!("sdfsdf {}", std::any::type_name::<T>())
// }

impl Engine {
    pub fn gogogo(&mut self) {
        for _ in 1..10 {
            for m in self.modules.iter_mut() {
                &m.borrow_mut().process();
            }
            for c in self.cables.iter_mut() {
                let mut input_m = c.input_module.as_ref().borrow_mut();
                let mut output_m = c.output_module.as_ref().borrow_mut();
                input_m.outputs()[c.output_port] = output_m.inputs()[c.input_port];
            }
        }
    }
}

impl Default for Engine{
    fn default() -> Self {
        let  mods: Vec<Rc<RefCell<dyn Module>>> = vec![Rc::new(RefCell::new(M1::default())),
                         Rc::new(RefCell::new(M2::default()))];
        let m1 = mods[0].clone();
        let m2 = mods[1].clone();

        Engine {
            modules: mods,
            cables: vec![Cable{
                input_module: m1,
                output_module: m2,
                input_port: 0,
                output_port: 0
            }]
        }
    }
}