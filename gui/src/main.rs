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
    let mut e = biser::test_engine();
    e.start();
    thread::sleep(Duration::from_millis(3000));
    e.stop();
}
