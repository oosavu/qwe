pub use std::sync::atomic::{AtomicBool, Ordering}; //todo
pub use std::sync::{Arc, Mutex,Condvar};
pub use std::{thread, time};
use std::ops::Add;
pub use std::ptr::NonNull;
use std::time::{Duration, SystemTime};
use crate::*;



const FALLBACK_FRAME_SIZE: usize = 64;


pub(crate) trait Module {
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<AudioPort>;
    fn outputs(&mut self) -> &mut Vec<AudioPort>;

    //fn hand_inputs(&mut self) -> &mut Vec<Port>;
    //fn hand_outputs(&mut self) -> &mut Vec<Port>;
}
type ModulePointer = Option<NonNull<dyn Module>>; //need to have nullable dynamic pointer

pub(crate) struct Cable {
    pub input_module_p: ModulePointer,
    pub output_module_p: ModulePointer,
    pub input_port: usize,
    pub output_port: usize,
}

pub(crate) trait DefaultModuleInterface{

}
type DefaultModulePointer = Option<NonNull<dyn DefaultModuleInterface>>;


pub(crate) trait DefaultModule{
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
    pub modules_pointers: Vec<ModulePointer>, //todo arc?
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
    modules: Vec<Mutex<Arc<dyn Module>>>,
    cables: Vec<Mutex<Cable>>,
    core: Arc<Mutex<RealTimeCore>>,

    fallback_mutex: Arc<(Mutex<bool>, Condvar)>,
    frame_rate: i64,



    fallback_handle: Option<thread::JoinHandle<()>>,
    fallback_alive: Arc<AtomicBool>, // alive of tread itself
}

impl Engine {
    pub fn start(&mut self){
        self.fallback_alive.store(true, Ordering::SeqCst);
        self.start_fallback();
    }

    pub fn stop(&mut self){
        self.stop_fallback();
    }

    //
    // pub fn add_module(&mut self, module: Mutex<Arc<dyn Module>>){
    //     self.modules.push(module)
    //     if
    // }

    fn start_fallback(&mut self) {
        dbg!("starting fallback...");
        let cor = self.core.clone();
        self.fallback_handle = Some(std::thread::spawn(move || {
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
        let cor = self.core.lock().unwrap();
    }


    fn get_pointer(mods: &mut Vec<Mutex<Arc<dyn Module>>>, i: usize) -> ModulePointer{ // get unsafe fat pointer
        return unsafe {
            Some(NonNull::new_unchecked(Arc::as_ptr(&mut *mods[i].lock().unwrap()) as *mut dyn Module))
        };
    }
}

pub fn test_engine() -> Engine {
    let mut mods: Vec<Mutex<Arc<dyn Module>>> = vec![Mutex::new(Arc::new(crate::sine::ModuleSine::default())),
                                                     Mutex::new(Arc::new(crate::audio_o::ModuleO::default()))];
    let alive: Arc<AtomicBool> = Arc::new(AtomicBool::default());
    let fallback_active: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(true), Condvar::new()));
    Engine {
        fallback_handle: None,
        fallback_alive: alive.clone(),
        core: Arc::new(std::sync::Mutex::new(RealTimeCore {
            modules_pointers: vec![Engine::get_pointer(&mut mods, 0), Engine::get_pointer(&mut mods, 1)],
            default_module: None,
            cable_core: vec![Cable {
                input_module_p: Engine::get_pointer(&mut mods, 0),
                output_module_p: Engine::get_pointer(&mut mods, 1),
                input_port: 0,
                output_port: 0,
            }],
            sample_rate: 96000,
            current_time: SystemTime::now(),
            alive: alive.clone(),
            is_fallback_active: fallback_active.clone()
        })),
        fallback_mutex: fallback_active.clone(),
        cables: vec![Mutex::new(Cable {
            input_module_p: Engine::get_pointer(&mut mods, 0),
            output_module_p: Engine::get_pointer(&mut mods, 1),
            input_port: 0,
            output_port: 0,
        })],
        modules: mods,
        frame_rate: 48000
    }
}