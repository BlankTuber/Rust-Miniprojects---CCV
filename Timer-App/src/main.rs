mod app;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Hello Eframe",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app::App::default()))),
    )
}
