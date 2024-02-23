use std::time::{Duration, Instant};

use super::canvas::Canvas;

pub struct GameState {
    pub canvas: Canvas,
    pub delta_time: Duration,
    last_frame_time: Instant,
}
impl GameState {
    pub fn new(canvas: Canvas) -> Self {
        Self {
            canvas,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
        }
    }

    pub fn set_time(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
    }
}
