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
        self.set_position(resolution, delta_time);
        self.set_velocity(resolution, delta_time);
    }

    fn set_position(&mut self, resolution: &Resolution, delta_time: &Duration) {
        let unclamped_position = self.velocity * delta_time.as_secs_f64() + self.position;
        let new_position = unclamped_position.clamp(
            Some(Vec2::new(0., 0.)),
            Some(Vec2::new((resolution.width - 1) as f64, (resolution.height - 1) as f64))
        );
        self.position = new_position;
    }

    fn set_velocity(&mut self, resolution: &Resolution, delta_time: &Duration) {
        let floor_collision = self.position.y == (resolution.height - 1) as f64 && self.velocity.y >= 0.;
        let ceil_collision = self.position.y == 0. && self.velocity.y < 0.;
        self.velocity.y = if floor_collision || ceil_collision {
            0.
        } else {
            self.velocity.y + GRAVITY * delta_time.as_secs_f64()
        };

        let right_wall_collision = self.position.x == (resolution.width - 1) as f64 && self.velocity.x > 0.;
        let left_wall_collision = self.position.x == 0. && self.velocity.x < 0.;
        self.velocity.x = if right_wall_collision || left_wall_collision {
            0.
        } else {
            self.velocity.x
        };
    }
}
