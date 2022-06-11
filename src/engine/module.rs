use std::rc::Rc;

const CHANELS: usize = 16;

struct Port{
    value: f32
}

trait Module{
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<Port>;
    fn outputs(&mut self) -> &mut Vec<Port>;
}

struct Cable{
    input_module:  Rc<dyn Module>,
    output_module: Rc<dyn Module>
}