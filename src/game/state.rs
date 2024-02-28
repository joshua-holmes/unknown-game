use std::time::{Duration, Instant};

use super::{
    canvas::Canvas, dot::Dot, geometry::Vec2, material::Material, INITIAL_CANVAS_RESOLUTION,
};

pub struct GameState {
    pub canvas: Canvas,
    pub delta_time: Duration,
    last_frame_time: Instant,
}
impl GameState {
    pub fn new() -> Self {
        let mut canvas = Canvas::new(INITIAL_CANVAS_RESOLUTION);

        // Set some dots for testing
        canvas.dots.push(Dot::new(
            Material::Sand,
            Vec2::new(0., 0.),
            Vec2::new(100., 0.),
        ));
        //
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
