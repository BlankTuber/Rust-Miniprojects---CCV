mod app;
mod state;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    // -------------------------------------------------------------------------
    // Window configuration
    // -------------------------------------------------------------------------
    // ViewportBuilder follows the builder pattern to configure the OS window.
    // Sizes are in egui's logical points, not physical pixels — egui scales
    // them automatically based on the display's DPI/scale factor.
    let window_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_position([100.0, 100.0])
            .with_min_inner_size([640.0, 360.0]),
        ..Default::default()
    };

    // -------------------------------------------------------------------------
    // App startup
    // -------------------------------------------------------------------------
    // run_native takes a closure that constructs our app. The closure receives a
    // CreationContext (cc) which provides the egui Context, storage handle, and
    // graphics context — useful for restoring saved state or loading fonts.
    // The Box<dyn App> return lets eframe own the app for its lifetime.

    eframe::run_native(
        "Timer",
        window_options,
        Box::new(|cc| Ok(Box::new(app::TimerApp::new(cc)))),
    )
}
