use crate::synth_core::*;
use crate::*;
pub struct ModuleSine {
    ins: Vec<AudioPort>,
    outs: Vec<AudioPort>,
    sample_clock: f32,
}

impl Module for ModuleSine {
    fn process(&mut self) {
        let sample_rate = 48000.0f32;
        self.sample_clock = (self.sample_clock + 1.0);

        self.outs[0].value[0] = (self.sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin();
        //println!("outso:{}", self.outs[0].value)
    }
    fn inputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.ins
    }
    fn outputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.outs
    }
}

impl Default for ModuleSine {
    fn default() -> Self {
        ModuleSine{
            ins: vec![],
            outs: AudioPort::create_audio_ports(1),
            sample_clock: 0.0
        }
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
