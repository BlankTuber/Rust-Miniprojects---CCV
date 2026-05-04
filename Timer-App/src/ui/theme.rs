use eframe::egui;

// -------------------------------------------------------------------------
// Theme application
// -------------------------------------------------------------------------
/// Applies the custom dark theme settings to the egui Context.
/// This is typically called once at startup before the first frame is rendered.
pub fn apply(ctx: &egui::Context) {
    // Set the base theme to Dark to ensure default elements match our vibe.
    ctx.set_theme(egui::Theme::Dark);

    // Globally override specific style properties, like padding and spacing,
    // to give the UI a more breathable, touch-friendly feel.
    ctx.all_styles_mut(|style| {
        style.spacing.button_padding = egui::vec2(20.0, 10.0);
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    });

    // Apply our custom color palette overrides on top of the base dark theme.
    ctx.set_visuals_of(egui::Theme::Dark, build_dark_visuals());
}

// -------------------------------------------------------------------------
// Custom Visuals
// -------------------------------------------------------------------------
/// Constructs a customized `egui::Visuals` struct with specific colors.
fn build_dark_visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();

    // Deepen the background colors to make the timer pop out more.
    visuals.panel_fill = egui::Color32::from_rgb(15, 15, 20);
    visuals.window_fill = egui::Color32::from_rgb(25, 25, 35);

    // Slightly soften the pure white text for better contrast readability.
    visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 235));

    // Define widget interaction states (inactive, hovered, active).
    // This gives interactive elements like buttons distinct visual feedback.
    visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(40, 40, 58);
    visuals.widgets.inactive.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 100));

    visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(60, 60, 85);
    visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(130, 130, 200));

    visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(80, 80, 120);
    visuals.widgets.active.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(160, 160, 255));

    visuals
}
