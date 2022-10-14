pub use std::sync::atomic::{AtomicBool, Ordering};
pub use std::sync::{Arc, Mutex};
pub use std::{thread, time};
use std::ops::Add;
pub use std::ptr::NonNull;
use std::time::{Duration, SystemTime};

//const CHANELS: usize = 16;
const TimeFrameSize: i64 = 64;

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
    pub sample_rate: i64,
    pub current_time: SystemTime,
    alive: Arc<AtomicBool>,
}

unsafe impl Send for RealTimeCore {}
unsafe impl Sync for RealTimeCore {}

impl RealTimeCore {
    pub fn compute_frame(&mut self) {
        unsafe {
           // let now = SystemTime::now();
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

impl Engine {
    pub fn start(&mut self) {
        let cor = self.core.clone();

        self.handle = Some(std::thread::spawn(move || {
            let mut alive = cor.lock().unwrap().alive.clone();
            alive.store(true, Ordering::SeqCst);
            {
                let mut cor = cor.lock().unwrap();
                cor.current_time = SystemTime::now();
            }
            let mut samples_count : i64 = 0;
            while alive.load(Ordering::SeqCst) {
                let mut cor = cor.lock().unwrap();
                let duration = cor.current_time.elapsed().unwrap().as_millis() as i64;
                let required_samples = cor.sample_rate * duration / 1000;
                if(required_samples < samples_count)
                {
                    let pause_millis = (samples_count - required_samples) * 1000i64 / cor.sample_rate;
                    //dbg!(pause_millis);
                    thread::sleep(time::Duration::from_millis(std::cmp::max(pause_millis as u64, 1)));
                    continue;
                }
                cor.compute_frame();
                samples_count = samples_count + TimeFrameSize;
                dbg!(samples_count);
                //thread::sleep(time::Duration::from_millis(10));
            }
        }));
    }

    pub fn stop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
        self.handle
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
        let cor = self.core.lock().unwrap();
        dbg!(cor.current_time.elapsed());
    }


    fn get_pointer(mods: &mut Vec<Mutex<Arc<dyn Module>>>, i: usize) -> ModulePointer{
        return unsafe {
            Some(NonNull::new_unchecked(Arc::as_ptr(&mut *mods[i].lock().unwrap()) as *mut dyn Module))
        };
    }
}

pub fn test_engine() -> Engine {
    let mut mods: Vec<Mutex<Arc<dyn Module>>> = vec![Mutex::new(Arc::new(crate::sine::ModuleSine::default())),
                                                     Mutex::new(Arc::new(crate::audio_o::ModuleO::default()))];
    let alive: Arc<AtomicBool> = Arc::new(AtomicBool::default());
    Engine {
        handle: None,
        alive: alive.clone(),
        core: Arc::new(std::sync::Mutex::new(RealTimeCore {
            modules_pointers: vec![Engine::get_pointer(&mut mods, 0), Engine::get_pointer(&mut mods, 1)],
            cable_core: vec![Cable {
                input_module_p: Engine::get_pointer(&mut mods, 0),
                output_module_p: Engine::get_pointer(&mut mods, 1),
                input_port: 0,
                output_port: 0,
            }],
            sample_rate: 48000,
            current_time: SystemTime::now(),
            alive: alive.clone(),
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