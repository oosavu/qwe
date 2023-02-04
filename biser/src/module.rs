use std::sync::{Arc, Mutex};
use std::ptr::NonNull;
use std::borrow::BorrowMut;
use crate::AudioPort;

pub type ModuleArc = Arc<Mutex<dyn Module>>;
pub type ModulePointer = Option<NonNull<dyn Module>>;

pub trait DefaultModuleInterface {
    fn recommended_sample_rate() -> Option<i32>;
}

//  Option<NonNull> for performance. can't use impl trate here due to dynamic linkage in future
type DefaultModulePointer = Option<NonNull<dyn DefaultModuleInterface>>;

pub trait Module {
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<AudioPort>;
    fn outputs(&mut self) -> &mut Vec<AudioPort>;
    //fn hand_inputs(&mut self) -> &mut Vec<Port>;
    //fn hand_outputs(&mut self) -> &mut Vec<Port>;
    fn defult_module_interface() -> DefaultModulePointer;

}

// extract unsafe fat pointer
fn extract_pointer(module: &mut ModuleArc) -> ModulePointer {
    return unsafe {
        let asd: &Mutex<dyn Module> = module.borrow_mut();
        let qwe: *mut dyn Module = &mut *asd.lock().unwrap() as *mut dyn Module;
        Some(NonNull::new_unchecked(qwe))
    };
}

pub fn extract_pointer_from_vec(mods: &mut Vec<ModuleArc>, i: usize) -> ModulePointer { // get unsafe fat pointer
    return extract_pointer(&mut mods[i])
}

//Specialized for audio only for executing worker thread in it
// macro_rules! is_default_module {
//     ($($t:ty),+ $(,)?) => ($(
//         impl DefaultModule for $t {
//             fn jobs(&self) -> Box<De> {
//                 &self.defa
//             }
//
//             fn jobs_mut(&mut self) -> &mut Vec<String> {
//                 &mut self.jobs
//             }
//         }
//     )+)