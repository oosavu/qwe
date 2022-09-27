extern crate anyhow;
extern crate cpal;
extern crate ringbuf;

use std::arch::x86_64::_rdrand32_step;
use crate::synth_core::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamError;

use ringbuf::{Consumer, Producer, RingBuffer};

pub struct ModuleO {
    ins: Vec<Port>,
    outs: Vec<Port>,
    host: cpal::Host,
    device: cpal::Device,
    config: cpal::StreamConfig,
    producer: Producer<f32>,
    stream: cpal::Stream,
}

impl Module for ModuleO {
    fn process(&mut self) {
        unsafe {
            static mut count_i: isize = 0;
            count_i = count_i + 1;
            dbg!(count_i);
            self.producer.push(self.ins[0].value);
        }
    }
    fn inputs(&mut self) -> &mut Vec<Port> {
        &mut self.ins
    }
    fn outputs(&mut self) -> &mut Vec<Port> {
        &mut self.outs
    }
}

impl ModuleO {
    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: cpal::Sample,
    {
        unsafe {
            static mut count: isize = 0;
            static mut count_samples: isize = 0;
            count = count + 1;
            for frame in output.chunks_mut(channels) {
                let value: T = cpal::Sample::from::<f32>(&next_sample());
                for sample in frame.iter_mut() {
                    *sample = value;
                    count_samples = count_samples + 1;
                }
            }
            println!("coumt: {} samples: {}", count, count_samples)
        }
    }
    fn error_fn(err: StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }

    fn data_fn(
        consumer: &mut Consumer<f32>,
        data: &mut [f32],
        calback_info: &cpal::OutputCallbackInfo,
    ) {
        unsafe {
            static mut count: isize = 0;
            static mut count_samples: usize = 0;
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
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }

            dbg!(count, count_samples);
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
        let buffer = RingBuffer::new(12345);
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
            )
            .unwrap();
        stream.play().unwrap();

        ModuleO {
            ins: vec![Port { value: 0.0 }],
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

//
// #[derive(Debug)]
// struct Opt {
//     device: String
// }
//
// impl Opt {
//     fn from_args() -> Self {
//         let app = clap::Command::new("beep").arg(arg!([DEVICE] "The audio device to use"));
//         let matches = app.get_matches();
//         let device = matches.value_of("DEVICE").unwrap_or("default").to_string();
//         Opt { device }
//     }
// }
//
// pub fn soundtest(){
//     // let mut e: Engine;
//     // e.gogogo();
//
//     // let opt = Opt::from_args();
//
//     let host = cpal::default_host();
//     let device = host.default_output_device();
//
//
//     let config = device.unwrap().default_output_config().unwrap();
//     println!("Default output config: {:?}", config);
//     run::<f32>(&device, &config.into());
//
// }
//
// pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
//     where
//         T: cpal::Sample,
// {
//     let sample_rate = config.sample_rate.0 as f32;
//     let channels = config.channels as usize;
//
//     // Produce a sinusoid of maximum amplitude.
//     let mut sample_clock = 0f32;
//     let mut next_value = move || {
//         sample_clock = (sample_clock + 1.0) % sample_rate;
//         (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
//     };
//
//     let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
//
//     let stream = device.build_output_stream(
//         config,
//         move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
//             write_data(data, channels, &mut next_value)
//         },
//         err_fn,
//     )?;
//     stream.play()?;
//
//     std::thread::sleep(std::time::Duration::from_millis(1000));
//
//     Ok(())
// }
