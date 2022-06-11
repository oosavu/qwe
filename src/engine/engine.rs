use std::rc::Rc;
use std::cell::RefCell;

struct Engine{
    modules: Vec<Rc<RefCell<dyn Module>>>,
    //modules: Vec<M1>,
    cables: Vec<Cable>
}

impl Engine{
    fn gogogo(&mut self){
        for _ in 1..10{
            for mut m in self.modules.iter_mut(){
                m.borrow_mut().process();
            }
            for c in self.cables.iter_mut(){
                c.output_module.inputs()[0].value = c.input_module.outputs()[0].value
            }
        }
    }
}