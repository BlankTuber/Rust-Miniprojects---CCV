use crate::config::AudioConfig;
use crate::processing::{CHUNK_SIZE, Processor};
use cpal::{
    Device, Stream,
    traits::{DeviceTrait, StreamTrait},
};
use ringbuf::{
    HeapRb,
    traits::{Consumer, Producer, Split},
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub fn build_and_run(
    input_device: Device,
    output_device: Device,
    audio_config: AudioConfig,
) -> (Stream, Stream) {
    let input_channels = audio_config.input_channels as usize;
    let output_channels = audio_config.output_channels as usize;

    // Input to processing
    let (mut raw_producer, mut raw_consumer) =
        HeapRb::<f32>::new(CHUNK_SIZE * input_channels * 8).split();

    // Processing to output
    let (mut processed_producer, mut processed_consumer) =
        HeapRb::<f32>::new(CHUNK_SIZE * output_channels * 8).split();

    let input_overruns = Arc::new(AtomicUsize::new(0));
    let input_overruns_clone = input_overruns.clone();

    let wake_pair = Arc::new((Mutex::new(false), Condvar::new()));
    let wake_pair_clone = wake_pair.clone();

    // --- Input stream ---
    let input_stream = input_device
        .build_input_stream::<f32, _, _>(
            &audio_config.input_config,
            move |data: &[f32], _| {
                let mut pushed_any = false;
                for &sample in data {
                    if raw_producer.try_push(sample).is_err() {
                        input_overruns_clone.fetch_add(1, Ordering::Relaxed);
                    } else {
                        pushed_any = true;
                    }
                }

                if pushed_any && let Ok(mut ready) = wake_pair_clone.0.try_lock() {
                    *ready = true;
                    wake_pair_clone.1.notify_one();
                }
            },
            |err| eprintln!("Input error: {err}"),
            None,
        )
        .expect("Failed to build input stream");

    let mut processor = Processor::new(
        audio_config.input_sample_rate,
        audio_config.output_sample_rate,
        audio_config.input_channels,
        audio_config.output_channels,
    );

    // --- Processing thread ---
    thread::spawn(move || {
        let samples_per_chunk = CHUNK_SIZE * input_channels;
        let mut accumulator: Vec<f32> = Vec::with_capacity(samples_per_chunk);
        let (lock, cvar) = &*wake_pair;

        loop {
            let overruns = input_overruns.swap(0, Ordering::Relaxed);
            if overruns > 0 {
                eprintln!("WARNING: Input overrun! Dropped {} samples.", overruns);
            }

            while let Some(sample) = raw_consumer.try_pop() {
                accumulator.push(sample);
            }

            if processor.needs_fixed_chunks() {
                while accumulator.len() >= samples_per_chunk {
                    for &sample in processor.process_chunk(&accumulator[..samples_per_chunk]) {
                        processed_producer.try_push(sample).ok();
                    }
                    accumulator.drain(..samples_per_chunk);
                }
            } else if !accumulator.is_empty() {
                for &sample in processor.process_chunk(&accumulator) {
                    processed_producer.try_push(sample).ok();
                }
                accumulator.clear();
            }

            let mut ready = lock.lock().unwrap();
            if !*ready && accumulator.len() < samples_per_chunk {
                ready = cvar
                    .wait_timeout(ready, std::time::Duration::from_millis(5))
                    .unwrap()
                    .0;
            }
            *ready = false;
        }
    });

    // --- Output stream ---
    let output_stream = output_device
        .build_output_stream::<f32, _, _>(
            &audio_config.output_config,
            move |data: &mut [f32], _| {
                for sample in data {
                    *sample = processed_consumer.try_pop().unwrap_or(0.0);
                }
            },
            |err| eprintln!("Output error: {err}"),
            None,
        )
        .expect("Failed to build output stream");

    input_stream.play().expect("Failed to start input stream");
    output_stream.play().expect("Failed to start output stream");

    (input_stream, output_stream)
}
