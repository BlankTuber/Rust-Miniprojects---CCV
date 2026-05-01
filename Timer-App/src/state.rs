use std::time::{Duration, Instant};

pub enum AppMsg {
    Start,
    Stop,
    Reset,
}

pub struct AppState {
    accumulated: Duration,
    running_since: Option<Instant>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            accumulated: Duration::ZERO,
            running_since: None,
        }
    }
}

impl AppState {
    pub fn is_running(&self) -> bool {
        self.running_since.is_some()
    }

    pub fn elapsed(&self) -> Duration {
        match self.running_since {
            Some(started_at) => self.accumulated + started_at.elapsed(),
            None => self.accumulated,
        }
    }

    pub fn handle(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::Start => {
                if !self.is_running() {
                    self.running_since = Some(Instant::now());
                }
            }
            AppMsg::Stop => {
                if let Some(started_at) = self.running_since.take() {
                    self.accumulated += started_at.elapsed();
                }
            }
            AppMsg::Reset => {
                self.accumulated = Duration::ZERO;
                self.running_since = None;
            }
        }
    }
}
