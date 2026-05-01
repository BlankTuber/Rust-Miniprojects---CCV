use eframe::egui;

pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);

    // all_styles_mut applies to both dark and light themes,
    // so spacing/layout changes persist regardless of theme
    ctx.all_styles_mut(|style| {
        style.spacing.button_padding = egui::vec2(20.0, 10.0);
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    });

    // set_visuals_of targets only the dark theme specifically
    ctx.set_visuals_of(egui::Theme::Dark, build_dark_visuals());
}

fn build_dark_visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = egui::Color32::from_rgb(15, 15, 20);
    visuals.window_fill = egui::Color32::from_rgb(25, 25, 35);
    visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 235));

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
