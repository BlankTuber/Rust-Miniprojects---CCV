use std::io::stdin;
use cpal::{Device, Host, traits::{DeviceTrait, HostTrait}};

pub fn select_input_device(host: &Host) -> Device {
    select_device("input", host.input_devices().expect("Failed to enumerate input devices"))
}

pub fn select_output_device(host: &Host) -> Device {
    select_device("output", host.output_devices().expect("Failed to enumerate output devices"))
}

fn select_device(label: &str, devices: impl Iterator<Item = Device>) -> Device {
    let devices: Vec<Device> = devices.collect();

    println!("\nAvailable {label} devices:");
    for (i, device) in devices.iter().enumerate() {
        println!("  {i} | {}", device_name(device));
    }

    println!("Select {label} device number:");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    let index: usize = input.trim().parse().expect("Input must be a number");

    devices
        .into_iter()
        .nth(index)
        .expect("Index out of range")
}

fn device_name(device: &Device) -> String {
    device
        .description()
        .expect("Failed to get device description")
        .name()
        .to_string()
}
