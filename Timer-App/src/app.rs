use eframe::egui;

pub struct App {
    name: String,
    count: i32,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Hello World");
            ui.text_edit_singleline(&mut self.name);
            if ui.button("Click me").clicked() {
                self.count += 1;
            }
            ui.label(format!("Clicked {} times.", self.count));
        });
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: String::from("Test"),
            count: 0,
        }
    }
}
