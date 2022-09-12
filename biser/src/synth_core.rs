pub use std::sync::atomic::{AtomicBool, Ordering};
pub use std::sync::{Arc, Mutex};
pub use std::{thread, time};
pub use std::ptr::NonNull;

//const CHANELS: usize = 16;

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

pub struct RealTimeCore {
    pub modules_pointers: Vec<ModulePointer>,
    pub cable_core: Vec<Cable>,
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
    pub handle: Option<std::thread::JoinHandle<()>>,
    pub alive: Arc<AtomicBool>,
    pub modules: Vec<Mutex<Arc<dyn Module>>>,
    pub cables: Vec<Mutex<Cable>>,
    pub core: Arc<Mutex<RealTimeCore>>,
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

pub fn getPointer(mods: &mut Vec<Mutex<Arc<dyn Module>>>, i: usize) -> ModulePointer{
    return unsafe {
        Some(NonNull::new_unchecked(Arc::as_ptr(&mut *mods[i].lock().unwrap()) as *mut dyn Module))
    };
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            handle: None,
            alive: Arc::new(Default::default()),
            core: Arc::new(std::sync::Mutex::new(RealTimeCore {
                modules_pointers: vec![],
                cable_core: vec![],
            })),
            cables: vec![Mutex::new(Cable{
                input_module_p: None,
                output_module_p: None,
                output_port: 0,
                input_port: 0
            })],
            modules: vec![]
        }
    }
}
