use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Sample, SampleFormat};

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(&config, read_stream::<f32>, err_fn),
        SampleFormat::I16 => device.build_input_stream(&config, read_stream::<i16>, err_fn),
        SampleFormat::U16 => device.build_input_stream(&config, read_stream::<u16>, err_fn),
    }.unwrap();

    stream.play();

    fn read_stream<T: Sample>(data: &[T], _: &cpal::InputCallbackInfo) {
        
    }
}
