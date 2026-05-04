use std::time::Duration;

use eframe::egui;

use crate::state::{AppMsg, AppState};
use crate::ui;

// -------------------------------------------------------------------------
// Repaint timing
// -------------------------------------------------------------------------
// When the timer is running we need to refresh the display roughly every
// centisecond (10 ms) to keep the centisecond digit accurate. Driving repaints
// faster than this wastes CPU without any visible benefit.
const CENTISECOND: Duration = Duration::from_millis(10);

pub struct TimerApp {
    state: AppState,
}

impl TimerApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Apply our custom theme before the first frame is drawn.
        // CreationContext gives us access to the egui Context at startup,
        // which is the only time we need to set global style/theme overrides.
        ui::theme::apply(&cc.egui_ctx);
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for TimerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // -------------------------------------------------------------------------
        // Message passing
        // -------------------------------------------------------------------------
        // egui is an immediate mode GUI: the entire UI is re-described every frame.
        // Widgets can't mutate state directly because we hold an immutable borrow
        // of `state` while drawing. Instead, widgets push AppMsg variants into this
        // local Vec and we apply them *after* the draw pass, once all borrows end.
        let mut messages = vec![];

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui::timer::draw(ui, &self.state, &mut messages);
        });

        // Process any messages from this frame's UI interactions.
        for msg in messages {
            self.state.handle(msg);
        }

        // -------------------------------------------------------------------------
        // Selective repaint scheduling
        // -------------------------------------------------------------------------
        // egui only repaints in response to input events or explicit repaint
        // requests. When the timer is running we must request repaints ourselves
        // so the display keeps ticking, but we skip this when stopped to avoid
        // burning CPU/battery at idle.
        if self.state.is_running() {
            ui.request_repaint_after(CENTISECOND);
        }
    }
}
