extern crate anyhow;
extern crate cpal;
extern crate ringbuf;

use std::arch::x86_64::_rdrand32_step;
use crate::synth_core::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamError;
use crate::*;

use ringbuf::{HeapConsumer, HeapProducer, HeapRb};
//use ringbuf::HeapRb;

pub struct ModuleO {
    ins: Vec<AudioPort>,
    outs: Vec<AudioPort>,
    host: cpal::Host,
    device: cpal::Device,
    config: cpal::StreamConfig,
    producer: HeapProducer<f32>,
    stream: cpal::Stream,
}

impl Module for ModuleO {
    fn process(&mut self) {
        unsafe {
            static mut count_i: isize = 0;
            count_i = count_i + 1;
//            dbg!(count_i);
            self.producer.push(self.ins[0].value[0]);
        }
    }
    fn inputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.ins
    }
    fn outputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.outs
    }
}

impl ModuleO {

    fn error_fn(err: StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }

    fn data_fn(
        consumer: &mut HeapConsumer<f32>,
        data: &mut [f32],
        calback_info: &cpal::OutputCallbackInfo,
    ) {
        unsafe {
            static mut count: isize = 0;
            static mut count_samples: usize = 0;
            static mut count_samples2: usize = 0;
            count = count + 1;
            //TODO can we memcpy?
            let mut input_fell_behind = false;
            //count_samples = count_samples + data.len();
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => {
                        //dbg!(s);
                        count_samples = count_samples + 1;
                        s
                    },
                    None => {
                        //println!("beha");
                        input_fell_behind = true;
                        count_samples2 = count_samples2 + 1;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }

            dbg!(count, count_samples, count_samples2);
        }
    }

    fn qweqwe(&mut self) {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
impl Default for ModuleO {
    fn default() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let buffer = HeapRb::new(12345);
        let (mut producer, mut consumer) = buffer.split();

        let config = device.default_output_config().unwrap().into();
        println!("Default output config: {:?}", &config);
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], output_device: &cpal::OutputCallbackInfo| {
                    Self::data_fn(&mut consumer, data, output_device)
                },
                &Self::error_fn,
                None
            )
            .unwrap();
        stream.play().unwrap();

        ModuleO {
            ins: AudioPort::create_audio_ports(1),
            outs: vec![],
            host: host,
            device: device,
            config: config,
            producer: producer,
            stream: stream,
        }
    }
}

impl Drop for ModuleO {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
        //self.stream.pause()
    }
}
