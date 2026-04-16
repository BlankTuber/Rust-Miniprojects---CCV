# Rust-Miniprojects - CCV
A few miniprojects to help myself learn each library related to a bigger project, specifically the "Capture Card Viewer" project.

---

## Projects

### 1. Settings & Logging

#### Crates
- [serde](https://crates.io/crates/serde)
- [toml](https://crates.io/crates/toml)
- [anyhow](https://crates.io/crates/anyhow)
- [tracing](https://crates.io/crates/tracing)
- [tracing-subscriber](https://crates.io/crates/tracing-subscriber)

#### What to make
A CLI tool that can save, load, and edit a `settings.toml` file. It must handle errors gracefully (e.g., using `anyhow` if the file is missing) and log background processes so I can see exactly what the application is doing under the hood.



### 2. Timer App

#### Crates
- [eframe](https://crates.io/crates/eframe)

#### What to make
A simple standalone desktop timer application using the `eframe` immediate-mode GUI. It needs a start button, a stop button, and must maintain its state while the screen redraws continuously. 



### 3. Capture Card Scan

#### Crates
- [nokhwa](https://crates.io/crates/nokhwa) 
*(Note: Do not use default features. Explicitly enable the correct OS-specific feature flags: `input-msmf` for Windows, `input-avfoundation` for Mac, or `input-v4l` for Linux).*

#### What to make
A CLI tool that lists all camera devices (capture card video output) and allows you to select one. 
- **The Core Task:** Grab a single frame from the selected device.
- **The Crucial Step:** Manually decode the raw camera format (like YUYV or MJPEG) into standard RGB.
- Save the resulting RGB frame in a `.jpg` format to the disk.



### 4. Audio Monitor Tool

#### Crates
- [cpal](https://crates.io/crates/cpal)
- [ringbuf](https://crates.io/crates/ringbuf)

#### What to make
A CLI tool that lists all input devices, allows you to select one, then lists all output devices, and allows you to select one.
- **The Core Task:** Play continuous audio from the selected input to the selected output using a lock-free `ringbuf` to pass data between the high-speed audio threads.
- **The Crucial Step:** Ensure the `StreamConfig` (sample rate and channels) perfectly matches between the input and output devices, otherwise the audio pitch will be severely distorted.



### 5. The Thread Stress Test (UI vs. Background Compute)

#### Crates
- [eframe](https://crates.io/crates/eframe)
- `std::sync::mpsc` (Rust Standard Library)

#### What to make
A UI app featuring a continuous animation (like a moving shape) and two buttons designed to test thread freezing. 
- **Button 1 (Main Thread):** Performs a massive, random CPU-heavy math calculation directly in the UI loop, causing the animation to completely freeze until it spits out a result.
- **Button 2 (Side Thread):** Sends a message via `mpsc` to a background thread to perform that exact same math calculation. The UI animation must remain completely smooth while waiting. Once the math is done, the background thread sends the result back to the UI via a second `mpsc` channel to be displayed.
