// use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use std::cell::RefCell;

const CHANELS: usize = 16;

struct Port {
    value: f32,
}

trait Module {
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<Port>;
    fn outputs(&mut self) -> &mut Vec<Port>;
}

struct Cable {
    input_module: Rc<dyn Module>,
    output_module: Rc<dyn Module>,
}
#[derive(Default)]
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
#[derive(Default)]
struct M2 {
    t: f32,
    ins: Vec<Port>,
    outs: Vec<Port>,
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

    //qwe: Rc<RefCell<dyn Module>>,
    cables: Vec<Cable>,
}
fn print_type_of<T>(_: &T) {
    println!("sdfsdf {}", std::any::type_name::<T>())
}

impl Engine {
    pub fn gogogo(&mut self) {
        //println!("sdfsdf ");
        for _ in 1..10 {
            // print_type_of(self);
            //println!("sdfsdf ");
            for m in self.modules.iter_mut() {
                print_type_of(&m.borrow_mut().process());
                //*m.as_ref().borrow_mut().process();
            }

           //  // let q = 1233;
           //  // let mut sdfsdf : RefCell<i32> = RefCell::new(23);
           //  // *sdfsdf.borrow_mut() = q;
           //
           //  // let q = 1233;
           //  // let mut sdfsdf : Rc<RefCell<i32>> = Rc::new(RefCell::new(23));
           //  // *sdfsdf.as_ref().borrow_mut() = q;
           //
           //  let q = 1233;
           //  let mut sdfsdf : Vec<Rc<RefCell<i32>>> = Vec::new();
           //  //let mut sdfsdf : Vec<Rc<i32>> = Vec::new();
           //  for m in sdfsdf.iter_mut() {
           //      //m.as_ref()
           //      *m.as_ref().borrow_mut() = 2;
           //      // let asd = m.as_ref();
           //      // //let qwe = asd.borrow_mut();
           //      // *asd.borrow_mut() = 2;
           //      // asd = q;
           //      //     let mut zxc = m.as_ref().get_mut();
           //      //     let sdf = zxc.process();
           //  }
           //
           //  // let mut sdfsdf : Vec<Rc<RefCell<dyn Module>>>;
           //  // for m in sdfsdf.iter_mut() {
           //  //     let mut zxc = m.as_ref().get_mut();
           //  //     let sdf = zxc.process();
           //  //    // sdf.process();
           //  //     // m.as_ref().get_mut().process();
           //  // }
           //
           //
           //  //let mut qwe: Rc<RefCell<dyn Module>> = Rc::new(RefCell::new(M1::default()));
           //  //let mut asd = qwe.get_mut();
           // // let mut zxc = qwe.as_ref().borrow_mut().process();
           //  //zxc.process();
           //  // *zxc.value = 23.0;
           //  // for m in &mut self.modules {
           //  //     m.get_mut().value = 23.0;
           //  // }
           //  // self.qwe.as_ref().
           //  // self.qwe.borrow_mut() = s;
           //  // for m in &mut self.modules {
           //  //     // let qwe: &mut Rc<RefCell<dyn Module>> = m.borrow_mut();
           //  //     *m.process();
           //  // }
           //  // for c in self.cables.iter_mut() {
           //  //     let v = *c.output_module;
           //  //     let v2 = v.borrow_mut().inputs();
           //  //         v[0].value = 23.0;
           //  //        // c.input_module.borrow_mut().outputs()[0].borrow_mut().value
           //  // }
        }
    }
}

impl Default for Engine{
    fn default() -> Self {
        Engine {
            modules: vec![Rc::new(RefCell::new(M1::default())),
                          Rc::new(RefCell::new(M2::default()))],
            cables: vec![]
        }
    }
}