use std::{fs::canonicalize, io::stdin};

use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    query,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};

fn main() {
    println!("Welcome to the Capture Card Frame Printer!");

    let cameras = query(nokhwa::utils::ApiBackend::MediaFoundation).unwrap();

    for camera in cameras {
        println!("Found camera: {}", camera.human_name());
        println!("Camera Index: {:?}", camera.index());
    }

    println!("\nSelect an index to capture a frame from!");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line!");

    let input_int: u32 = input.trim().parse().expect("Input was not a valid int");

    let selected_camera = CameraIndex::Index(input_int);

    let req_format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::None);

    let mut created_camera = Camera::new(selected_camera, req_format).unwrap();
    created_camera.open_stream().unwrap();

    for _ in 0..5 {
        created_camera.frame().unwrap();
    }

    let frame = created_camera.frame().unwrap();

    let decoded = frame.decode_image::<RgbFormat>().unwrap();
    decoded.save("frame.jpg").unwrap();

    let abs_path = canonicalize("frame.jpg").unwrap();

    println!("Decoded frame at: {}", abs_path.display());
}
