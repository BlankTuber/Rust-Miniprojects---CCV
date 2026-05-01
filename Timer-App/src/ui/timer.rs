use eframe::egui;

use crate::state::{AppMsg, AppState};

pub fn draw(ui: &mut egui::Ui, state: &AppState, messages: &mut Vec<AppMsg>) {
    // Center the content vertically by splitting remaining space into
    // a top spacer and the content itself, rather than using a fixed percentage
    let content_height = measure_content_height(ui);
    let top_padding = ((ui.available_height() - content_height) / 2.0).max(8.0);

    ui.add_space(top_padding);
    ui.vertical_centered(|ui| {
        draw_time_display(ui, state);
        ui.add_space(24.0);
        draw_buttons(ui, state, messages);
    });
}

fn measure_content_height(ui: &egui::Ui) -> f32 {
    let time_display_height = 72.0 + ui.style().spacing.item_spacing.y;
    let spacer_height = 24.0;
    let button_height =
        ui.style().spacing.button_padding.y * 2.0 + ui.text_style_height(&egui::TextStyle::Button);

    time_display_height + spacer_height + button_height
}

fn draw_time_display(ui: &mut egui::Ui, state: &AppState) {
    let elapsed = state.elapsed();
    let minutes = elapsed.as_secs() / 60;
    let seconds = elapsed.as_secs() % 60;
    let centiseconds = elapsed.subsec_millis() / 10;

    ui.label(
        egui::RichText::new(format!("{:02}:{:02}.{:02}", minutes, seconds, centiseconds))
            .size(72.0)
            .monospace()
            .strong(),
    );
}

fn draw_buttons(ui: &mut egui::Ui, state: &AppState, messages: &mut Vec<AppMsg>) {
    ui.horizontal(|ui| {
        let (start_or_stop_label, start_or_stop_msg) = if state.is_running() {
            ("Stop", AppMsg::Stop)
        } else {
            ("Start", AppMsg::Start)
        };

        if ui.button(start_or_stop_label).clicked() {
            messages.push(start_or_stop_msg);
        }
        if ui.button("Reset").clicked() {
            messages.push(AppMsg::Reset);
        }
    });
}
