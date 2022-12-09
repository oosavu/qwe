pub use std::sync::atomic::{AtomicBool, Ordering};
//todo
pub use std::sync::{Arc, Condvar, Mutex};
pub use std::{thread, time};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;
pub use std::ptr::NonNull;
use std::time::{Duration, SystemTime};
use crate::*;
use crate::module::{Module, ModulePointer};

const FALLBACK_FRAME_SIZE: usize = 64; //need to have nullable dynamic pointer
pub(crate) struct Cable {
    pub input_module_p: ModulePointer,
    pub output_module_p: ModulePointer,
    pub input_port: usize,
    pub output_port: usize,
}

pub(crate) trait DefaultModuleInterface {}

type DefaultModulePointer = Option<NonNull<dyn DefaultModuleInterface>>;


pub(crate) trait DefaultModule {
    fn defult_module_interface() -> DefaultModulePointer;
}

//Specialized for audio only for executing worker thread in it
macro_rules! is_default_module {
    ($($t:ty),+ $(,)?) => ($(
        impl DefaultModule for $t {
            fn jobs(&self) -> Box<De> {
                &self.defa
            }

            fn jobs_mut(&mut self) -> &mut Vec<String> {
                &mut self.jobs
            }
        }
    )+)
}

struct RealTimeCore {
    pub modules_pointers: Vec<ModulePointer>,
    //todo arc?
    pub default_module: DefaultModulePointer,
    pub cable_core: Vec<Cable>,
    pub sample_rate: i64,
    pub current_time: SystemTime,
    alive: Arc<AtomicBool>,
    is_fallback_active: Arc<(Mutex<bool>, Condvar)>,
}

unsafe impl Send for RealTimeCore {}

unsafe impl Sync for RealTimeCore {}

impl RealTimeCore {
    pub fn compute_frame(&mut self, time_frame: usize) { // TODO does it vectorized automativally?
        unsafe {
            for _ in 0..time_frame {
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
    modules: Vec<Arc<Mutex<dyn Module>>>,
    cables: Vec<Mutex<Cable>>,
    core: Arc<Mutex<RealTimeCore>>,

    fallback_mutex: Arc<(Mutex<bool>, Condvar)>,
    frame_rate: i64,

    fallback_handle: Option<thread::JoinHandle<()>>,
    fallback_alive: Arc<AtomicBool>, // alive of tread itself
}

impl Engine {
    pub fn start(&mut self) {
        self.fallback_alive.store(true, Ordering::SeqCst);
        self.start_fallback();
    }

    pub fn stop(&mut self) {
        self.stop_fallback();
    }

    //
    // pub fn add_module(&mut self, module: Arc<Mutex<dyn Module>>){
    //     self.modules.push(module);
    //     let cor = self.core.lock().unwrap();
    //    // cor.modules_pointers.append()
    // }

    fn start_fallback(&mut self) {
        dbg!("starting fallback...");
        let cor = self.core.clone();
        self.fallback_handle = Some(thread::spawn(move || {
            let mut alive = cor.lock().unwrap().alive.clone();
            alive.store(true, Ordering::SeqCst);
            {
                let mut cor = cor.lock().unwrap();
                cor.current_time = SystemTime::now();
            }
            let mut samples_count: i64 = 0;
            while alive.load(Ordering::SeqCst) {
                let mut cor = cor.lock().unwrap();
                let duration = cor.current_time.elapsed().unwrap().as_millis() as i64;
                let required_samples = cor.sample_rate * duration / 1000;
                if required_samples < samples_count
                {
                    let pause_millis = (samples_count - required_samples) * 1000i64 / cor.sample_rate;
                    //dbg!(pause_millis);
                    thread::sleep(Duration::from_millis(std::cmp::max(pause_millis as u64, 1)));
                    continue;
                }
                cor.compute_frame(FALLBACK_FRAME_SIZE);
                samples_count = samples_count + FALLBACK_FRAME_SIZE as i64;
                dbg!(samples_count);
                //thread::sleep(time::Duration::from_millis(10));
            }
        }));
    }

    fn stop_fallback(&mut self) {
        self.fallback_alive.store(false, Ordering::SeqCst);
        self.fallback_handle
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
        let cor = self.core.lock().unwrap();
        dbg!("fallback stopped. working time: {}", cor.current_time.elapsed());
    }

    fn pause_fallback(&mut self) {
        self.fallback_alive.store(false, Ordering::SeqCst);
        self.fallback_handle
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
        //let cor = self.core.lock().unwrap();
    }
}

impl Default for Engine{
    fn default() -> Self {
        Engine{
            modules: vec![],
            cables: vec![],
            core: Arc::new(Mutex::new(RealTimeCore {
                modules_pointers: vec![],
                default_module: None,
                cable_core: vec![],
                sample_rate: 0,
                current_time: SystemTime::now(),
                alive: Arc::new(Default::default()),
                is_fallback_active: Arc::new((Mutex::new(false), Default::default()))
            })),
            fallback_mutex: Arc::new((Mutex::new(false), Default::default())),
            frame_rate: 0,
            fallback_handle: None,
            fallback_alive: Arc::new(Default::default())
        }
    }
}

pub fn test_engine() -> Engine {
    let mut mods: Vec<Arc<Mutex<dyn Module>>> = vec![Arc::new(Mutex::new(sine::ModuleSine::default())),
                                                     Arc::new(Mutex::new(audio_o::ModuleO::default()))];
    let alive: Arc<AtomicBool> = Arc::new(AtomicBool::default());
    let fallback_active: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(true), Condvar::new()));
    Engine {
        fallback_handle: None,
        fallback_alive: alive.clone(),
        core: Arc::new(Mutex::new(RealTimeCore {
            modules_pointers: vec![crate::module::extract_pointer_from_vec(&mut mods, 0), crate::module::extract_pointer_from_vec(&mut mods, 1)],
            default_module: None,
            cable_core: vec![Cable {
                input_module_p: crate::module::extract_pointer_from_vec(&mut mods, 0),
                output_module_p: crate::module::extract_pointer_from_vec(&mut mods, 1),
                input_port: 0,
                output_port: 0,
            }],
            sample_rate: 96000,
            current_time: SystemTime::now(),
            alive: alive.clone(),
            is_fallback_active: fallback_active.clone(),
        })),
        fallback_mutex: fallback_active.clone(),
        cables: vec![Mutex::new(Cable {
            input_module_p: crate::module::extract_pointer_from_vec(&mut mods, 0),
            output_module_p: crate::module::extract_pointer_from_vec(&mut mods, 1),
            input_port: 0,
            output_port: 0,
        })],
        modules: mods,
        frame_rate: 48000,
    }
}