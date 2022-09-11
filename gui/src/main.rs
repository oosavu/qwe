extern crate biser;
use std::{thread, time::Duration};

slint::slint!{
    HelloWorld := Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() {
    //biser::soundtest();
    let mut e = biser::Engine::default();
    e.start();
    thread::sleep(Duration::from_millis(1000));
    e.stop();
}
