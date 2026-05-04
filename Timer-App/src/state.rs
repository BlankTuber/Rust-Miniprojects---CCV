use std::time::{Duration, Instant};

// -------------------------------------------------------------------------
// Messages
// -------------------------------------------------------------------------
// AppMsg is the set of things the UI can request the state to do.
// Using an enum keeps UI code free of direct state mutation — it just
// describes *intent*, and AppState decides what to do with it.
pub enum AppMsg {
    Start,
    Stop,
    Reset,
}

// -------------------------------------------------------------------------
// State
// -------------------------------------------------------------------------
// Time is tracked in two parts:
//
//   `accumulated`    — total duration from all previous (stopped) runs
//   `running_since`  — the Instant the current run started, or None if stopped
//
// This split avoids storing a mutable "last elapsed" that would need updating
// every frame. Instead, `elapsed()` computes the live total on demand by adding
// the in-progress run's wall-clock duration to the accumulated total.
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
    /// Returns true if the timer is currently counting up.
    pub fn is_running(&self) -> bool {
        self.running_since.is_some()
    }

    /// Returns the total elapsed time, including any currently-running segment.
    ///
    /// Called every frame while running, so it must be cheap — the addition of
    /// two Durations is all the work done here.
    pub fn elapsed(&self) -> Duration {
        match self.running_since {
            Some(started_at) => self.accumulated + started_at.elapsed(),
            None => self.accumulated,
        }
    }

    // -------------------------------------------------------------------------
    // State transitions
    // -------------------------------------------------------------------------
    pub fn handle(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::Start => {
                if !self.is_running() {
                    // Record the moment we started so elapsed() can compute
                    // time-since-start on every subsequent frame.
                    self.running_since = Some(Instant::now());
                }
            }
            AppMsg::Stop => {
                // take() removes the value from the Option and returns it,
                // atomically leaving None behind — no separate clear needed.
                if let Some(started_at) = self.running_since.take() {
                    // Fold the just-ended run into the accumulated total so it
                    // survives the next Start/Stop cycle.
                    self.accumulated += started_at.elapsed();
                }
            }
            AppMsg::Reset => {
                // Clear both halves of the time state unconditionally.
                // This works whether the timer is running or stopped.
                self.accumulated = Duration::ZERO;
                self.running_since = None;
            }
        }
    }
}
