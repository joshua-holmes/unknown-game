use std::time::Duration;

use crate::rendering::glsl_types::Resolution;

use super::{geometry::Vec2, GRAVITY, material::Material};

#[derive(Debug)]
pub struct Dot {
    pub material: Material,
    pub position: Vec2<f64>,
    pub velocity: Vec2<f64>,
}
impl Dot {
    pub fn new(material: Material, position: Vec2<f64>) -> Self {
        Self {
            material,
            position,
            velocity: Vec2::new(0., 0.),
        }
    }

    pub fn set_next_frame(&mut self, resolution: &Resolution, delta_time: &Duration) {
        self.position = (self.velocity * delta_time.as_secs_f64() + self.position).clamp(
            Some(Vec2::new(0., 0.)),
            Some(Vec2::new((resolution.width - 1) as f64, (resolution.height - 1) as f64))
        );
        self.velocity.y += GRAVITY * delta_time.as_secs_f64();
    }
}
