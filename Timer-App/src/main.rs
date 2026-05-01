mod app;
mod state;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let window_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 340.0])
            .with_resizable(true)
            .with_min_inner_size([360.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Timer",
        window_options,
        Box::new(|cc| Ok(Box::new(app::TimerApp::new(cc)))),
    )
}
