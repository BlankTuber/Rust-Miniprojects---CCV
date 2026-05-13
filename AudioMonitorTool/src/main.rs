mod config;
mod devices;
mod processing;
mod stream;

fn main() {
    let host = cpal::default_host();

    let input_device = devices::select_input_device(&host);
    let output_device = devices::select_output_device(&host);

    let audio_config = config::negotiate_configs(&input_device, &output_device);

    let (_input_stream, _output_stream) =
        stream::build_and_run(input_device, output_device, audio_config);

    println!("Monitoring started — press Ctrl+C to stop.");
    std::thread::sleep(std::time::Duration::MAX);
}
