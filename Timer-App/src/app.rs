use std::time::Duration;

use eframe::egui;

use crate::state::{AppMsg, AppState};
use crate::ui;

const CENTISECOND: Duration = Duration::from_millis(10);

pub struct TimerApp {
    state: AppState,
}

impl TimerApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        ui::theme::apply(&cc.egui_ctx);
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for TimerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut messages = vec![];

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui::timer::draw(ui, &self.state, &mut messages);
        });

        for msg in messages {
            self.state.handle(msg);
        }

        // Only schedule the next repaint when the display will actually change,
        // instead of requesting one every frame unconditionally
        if self.state.is_running() {
            ui.request_repaint_after(CENTISECOND);
        }
    }
}
