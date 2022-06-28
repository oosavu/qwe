extern crate biser;

slint::slint!{
    HelloWorld := Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() {
    biser::soundtest();
    let mut e: biser::Engine = biser::Engine::default();
    e.gogogo();

    // HelloWorld::new().run();
}

//
// impl Engine {
//     pub fn gogogo(&mut self) {
//         for _ in 1..10 {
//             for m in self.modules {
//                 *m.as_ref().borrow_mut().process();
//             }
//
//             for c in self.cables {
//                 let v = *c.output_module;
//                 let v2 = v.borrow_mut().inputs();
//                 v[0].value = 23.0;
//                 let input_m = c.input_module.as_ref().borrow_mut();
//                 let output_m = c.output_module.as_ref().borrow_mut();
//
//                 input_m.inputs()[c.input_port] = 0;
//                 // c.input_module.borrow_mut().outputs()[0].borrow_mut().value
//             }