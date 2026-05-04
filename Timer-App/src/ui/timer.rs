use eframe::egui;

use crate::state::{AppMsg, AppState};

// -------------------------------------------------------------------------
// Main UI Layout
// -------------------------------------------------------------------------
/// The primary entry point for drawing the timer interface.
/// It coordinates the layout, pushing the content down to center it vertically.
pub fn draw(ui: &mut egui::Ui, state: &AppState, messages: &mut Vec<AppMsg>) {
    // Calculate how much vertical space the content actually needs
    // so we can dynamically center it within the available window area.
    let content_height = measure_content_height(ui);
    let top_padding = ((ui.available_height() - content_height) / 2.0).max(8.0);

    ui.add_space(top_padding);
    ui.vertical_centered(|ui| {
        draw_time_display(ui, state);
        ui.add_space(24.0);
        draw_buttons(ui, state, messages);
    });
}

// -------------------------------------------------------------------------
// Sizing Helpers
// -------------------------------------------------------------------------
/// Estimates the total vertical height of the timer and buttons.
/// This is necessary for manual centering because egui doesn't inherently
/// center items vertically across the entire panel out-of-the-box.
fn measure_content_height(ui: &egui::Ui) -> f32 {
    let time_display_height = 72.0 + ui.style().spacing.item_spacing.y;
    let spacer_height = 24.0;
    let button_height =
        ui.style().spacing.button_padding.y * 2.0 + ui.text_style_height(&egui::TextStyle::Button);

    time_display_height + spacer_height + button_height
}

// -------------------------------------------------------------------------
// Component Rendering
// -------------------------------------------------------------------------
/// Renders the large digital time readout.
fn draw_time_display(ui: &mut egui::Ui, state: &AppState) {
    let elapsed = state.elapsed();

    // Break the total duration down into standard stopwatch components.
    // We only show seconds and centiseconds here.
    let seconds = elapsed.as_secs() % 60;
    let centiseconds = elapsed.subsec_millis() / 10;

    // Use a large, monospace font to prevent layout jitter as the numbers cycle.
    ui.label(
        egui::RichText::new(format!("{:02}.{:02}", seconds, centiseconds))
            .size(72.0)
            .monospace()
            .strong(),
    );
}

/// Renders the control buttons (Start/Stop and Reset).
fn draw_buttons(ui: &mut egui::Ui, state: &AppState, messages: &mut Vec<AppMsg>) {
    // Determine what the primary action button should do and say based on state.
    let (label, msg) = if state.is_running() {
        ("Stop", AppMsg::Stop)
    } else {
        ("Start", AppMsg::Start)
    };

    // Helper closure to measure how wide a button will be based on its text
    // and the current style settings.
    let measure_btn = |text: &str| -> f32 {
        let font_id = egui::TextStyle::Button.resolve(ui.style());
        let text_w = ui.fonts_mut(|f| {
            f.layout_no_wrap(text.to_string(), font_id, egui::Color32::WHITE)
                .size()
                .x
        });
        text_w + ui.style().spacing.button_padding.x * 2.0
    };

    // Calculate the total width of the button row so we can center it manually.
    let total = measure_btn(label) + measure_btn("Reset") + ui.style().spacing.item_spacing.x;

    ui.horizontal(|ui| {
        // Push everything to the right by half the remaining space to center the block.
        let pad = ((ui.available_width() - total) / 2.0).max(0.0);
        ui.add_space(pad);

        // Draw the buttons and record any click intents.
        if ui.button(label).clicked() {
            messages.push(msg);
        }
        if ui.button("Reset").clicked() {
            messages.push(AppMsg::Reset);
        }
    });
}
