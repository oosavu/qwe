use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::ptr::NonNull;

//const CHANELS: usize = 16;

#[derive(Clone, Copy)]
struct Port {
    value: f32,
}

trait Module {
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<Port>;
    fn outputs(&mut self) -> &mut Vec<Port>;
}
type ModulePointer = Option<NonNull<dyn Module>>;

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


struct Cable {
    input_module_p: ModulePointer,
    output_module_p: ModulePointer,
    input_port: usize,
    output_port: usize,
}

pub struct RealTimeCore {
    modules_pointers: Vec<ModulePointer>,
    cable_core: Vec<Cable>,
}

unsafe impl Send for RealTimeCore {}
unsafe impl Sync for RealTimeCore {}

impl RealTimeCore {
    pub fn gogogo(&mut self) {
        unsafe {
            for _ in 1..4 {
                for m in self.modules_pointers.iter_mut() {
                    let qwe = &mut *m.unwrap().as_mut();
                    qwe.process();
                }
                for c in self.cable_core.iter_mut() {
                    let input_m = &mut *c.input_module_p.unwrap().as_mut();
                    let output_m = &mut *c.output_module_p.unwrap().as_mut();
                    input_m.outputs()[c.output_port] = output_m.inputs()[c.input_port];
                }
            }
        }
    }
}

pub struct Engine {
    handle: Option<std::thread::JoinHandle<()>>,
    alive: Arc<AtomicBool>,
    modules: Vec<Mutex<Arc<dyn Module>>>,
    cables: Vec<Mutex<Cable>>,
    core: Arc<Mutex<RealTimeCore>>,
}

// fn print_type_of<T>(_: &T) {
//     println!("sdfsdf {}", std::any::type_name::<T>())
// }


impl Engine {
    pub fn start(&mut self) {//-> std::thread::JoinHandle<()>{
        let cor = self.core.clone();
        let alive = self.alive.clone();
        self.handle = Some(std::thread::spawn(move || {
            alive.store(true, Ordering::SeqCst);
            while alive.load(Ordering::SeqCst) {
                let mut cor = cor.lock().unwrap();//.expect("can't get mut");
                cor.gogogo();
                thread::sleep(time::Duration::from_millis(10));
            }
        }));
    }

    pub fn stop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
        self.handle
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
    }
}

fn getPointer(mods: &mut Vec<Mutex<Arc<dyn Module>>>, i: usize) -> ModulePointer{
    return unsafe {
        Some(NonNull::new_unchecked(Arc::as_ptr(&mut *mods[i].lock().unwrap()) as *mut dyn Module))
    };
}

impl Default for Engine {
    fn default() -> Self {
        let mut mods: Vec<Mutex<Arc<dyn Module>>> = vec![Mutex::new(Arc::new(M1::default())),
                                                     Mutex::new(Arc::new(M2::default()))];

       // let fff: *const dyn Module = Arc::as_ptr(&mut *mods[0].lock().unwrap());//asd as *mut dyn Module;
       //  let qwe = unsafe {
       //      Some(NonNull::new_unchecked(Arc::as_ptr(&mut *mods[0].lock().unwrap()) as *mut dyn Module))
       //  };
        // let sss =  Option::new(unsafe{ NonNull::new_unchecked(*&mods[0].as_ref());});
        // let mut qwe = Option::new(qweqw);
        //let m1p: *const dyn Module =  Arc::get_mut_unchecked(&qwe);

        //let m2p: *mut dyn Module = mods[1].as_ptr();
        //let qweqwe: *mut dyn Module = std::ptr::from_raw_parts() null_mut::<dyn Module>();


        Engine {
            handle: None,
            alive: Arc::new(Default::default()),
            core: Arc::new(std::sync::Mutex::new(RealTimeCore {
                modules_pointers: vec![getPointer(&mut mods, 0), getPointer(&mut mods, 1)],
                cable_core: vec![],
            })),
            cables: vec![Mutex::new(Cable{
                input_module_p: getPointer(&mut mods, 0),
                output_module_p: getPointer(&mut mods, 1),
                input_port: 0,
                output_port: 0
            })],
            modules: mods,
        }
    }
}
