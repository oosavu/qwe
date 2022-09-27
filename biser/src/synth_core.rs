pub use std::sync::atomic::{AtomicBool, Ordering};
pub use std::sync::{Arc, Mutex};
pub use std::{thread, time};
pub use std::ptr::NonNull;

//const CHANELS: usize = 16;
const TimeFrameSize: usize = 64;

#[derive(Clone, Copy)]
pub struct Port {
    pub value: f32,
}

pub trait Module {
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<Port>;
    fn outputs(&mut self) -> &mut Vec<Port>;
}
type ModulePointer = Option<NonNull<dyn Module>>;

pub struct Cable {
    pub input_module_p: ModulePointer,
    pub output_module_p: ModulePointer,
    pub input_port: usize,
    pub output_port: usize,
}

struct RealTimeCore {
    pub modules_pointers: Vec<ModulePointer>,
    pub cable_core: Vec<Cable>,
}

unsafe impl Send for RealTimeCore {}
unsafe impl Sync for RealTimeCore {}

impl RealTimeCore {
    pub fn compute_frame(&mut self) {
        unsafe {
            // let mut qweqwe = 123;
            // //qweqwe = 123123;
            for _ in 0..TimeFrameSize {
                for m in self.modules_pointers.iter_mut() {
                    let qwe = &mut *m.unwrap().as_mut();
                    qwe.process();
                }
                for c in self.cable_core.iter_mut() {
                    let input_m = &mut *c.input_module_p.unwrap().as_mut();
                    let output_m = &mut *c.output_module_p.unwrap().as_mut();
                    output_m.inputs()[c.output_port].value = input_m.outputs()[c.input_port].value;
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
                cor.compute_frame();
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


    fn get_pointer(mods: &mut Vec<Mutex<Arc<dyn Module>>>, i: usize) -> ModulePointer{
        return unsafe {
            Some(NonNull::new_unchecked(Arc::as_ptr(&mut *mods[i].lock().unwrap()) as *mut dyn Module))
        };
    }
}

//
// impl Default for Engine {
//     fn default() -> Self {
//         Engine {
//             handle: None,
//             alive: Arc::new(Default::default()),
//             core: Arc::new(std::sync::Mutex::new(RealTimeCore {
//                 modules_pointers: vec![],
//                 cable_core: vec![],
//             })),
//             cables: vec![Mutex::new(Cable{
//                 input_module_p: None,
//                 output_module_p: None,
//                 output_port: 0,
//                 input_port: 0
//             })],
//             modules: vec![]
//         }
//     }
// }



pub fn test_engine() -> Engine {
    let mut mods: Vec<Mutex<Arc<dyn Module>>> = vec![Mutex::new(Arc::new(crate::sine::ModuleSine::default())),
                                                     Mutex::new(Arc::new(crate::audio_o::ModuleO::default()))];

    Engine {
        handle: None,
        alive: Arc::new(Default::default()),
        core: Arc::new(std::sync::Mutex::new(RealTimeCore {
            modules_pointers: vec![Engine::get_pointer(&mut mods, 0), Engine::get_pointer(&mut mods, 1)],
            cable_core: vec![Cable {
                input_module_p: Engine::get_pointer(&mut mods, 0),
                output_module_p: Engine::get_pointer(&mut mods, 1),
                input_port: 0,
                output_port: 0,
            }],
        })),
        cables: vec![Mutex::new(Cable {
            input_module_p: Engine::get_pointer(&mut mods, 0),
            output_module_p: Engine::get_pointer(&mut mods, 1),
            input_port: 0,
            output_port: 0,
        })],
        modules: mods,
    }
}