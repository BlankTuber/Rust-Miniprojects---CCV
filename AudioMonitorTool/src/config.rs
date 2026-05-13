use cpal::traits::DeviceTrait;
use cpal::{Device, StreamConfig};

pub struct AudioConfig {
    pub input_config: StreamConfig,
    pub output_config: StreamConfig,
    pub input_channels: u16,
    pub output_channels: u16,
    pub input_sample_rate: u32,
    pub output_sample_rate: u32,
}

pub fn negotiate_configs(input_device: &Device, output_device: &Device) -> AudioConfig {
    let input = input_device
        .default_input_config()
        .expect("Failed to get default input config");

    let output = output_device
        .default_output_config()
        .expect("Failed to get default output config");

    let input_channels = input.channels();
    let output_channels = output.channels();
    let input_sample_rate = input.sample_rate();
    let output_sample_rate = output.sample_rate();

    println!("Input:  {input_channels}ch @ {input_sample_rate}Hz");
    println!("Output: {output_channels}ch @ {output_sample_rate}Hz");

    AudioConfig {
        input_config: input.config(),
        output_config: output.config(),
        input_channels,
        output_channels,
        input_sample_rate,
        output_sample_rate,
    }
}
