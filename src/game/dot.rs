use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::{rendering::glsl_types::Resolution, game::rng};

use super::{geometry::Vec2, material::Material, GRAVITY};

#[derive(Debug)]
pub struct Dot {
    pub material: Material,
    pub position: Vec2<f64>,
    pub velocity: Vec2<f64>,
    last_offset: Instant,
}
impl Dot {
    pub fn new(material: Material, position: Vec2<f64>, velocity: Vec2<f64>) -> Self {
        Self {
            material,
            position,
            velocity,
            last_offset: Instant::now(),
        }
    }

    pub fn set_next_frame(&mut self, resolution: &Resolution, delta_time: &Duration) {
        self.set_position(resolution, delta_time);
        self.set_velocity(resolution, delta_time);
    }

    fn set_position(&mut self, resolution: &Resolution, delta_time: &Duration) {
        let offset_from_drag = self.calculate_pos_offset_from_drag();
        let unclamped_position =
            self.velocity * delta_time.as_secs_f64() + offset_from_drag + self.position;
        let new_position = unclamped_position.clamp(
            Some(Vec2::new(0., 0.)),
            Some(Vec2::new(
                (resolution.width - 1) as f64,
                (resolution.height - 1) as f64,
            )),
        );
        self.position = new_position;
    }

    fn set_velocity(&mut self, resolution: &Resolution, delta_time: &Duration) {
        let accel = self.calculate_real_drag() + GRAVITY;
        let mut new_velocity = self.velocity + (accel * delta_time.as_secs_f64());

        let floor_collision =
            self.position.y == (resolution.height - 1) as f64 && self.velocity.y >= 0.;
        let ceil_collision = self.position.y == 0. && self.velocity.y < 0.;
        if floor_collision || ceil_collision {
            new_velocity.y = 0.;
        }

        let right_wall_collision =
            self.position.x == (resolution.width - 1) as f64 && self.velocity.x > 0.;
        let left_wall_collision = self.position.x == 0. && self.velocity.x < 0.;
        if right_wall_collision || left_wall_collision {
            new_velocity.x = 0.;
        }

        self.velocity = new_velocity;
    }

    /// Calculates real change in acceleration from drag property
    fn calculate_real_drag(&self) -> Vec2<f64> {
        let drag_value = self.material.properties().drag;
        let vel_value = self.velocity.pythagorean_theorem();
        if vel_value == 0. {
            return Vec2::new(0., 0.);
        }
        let ratio = drag_value / vel_value;
        self.velocity * -ratio
    }

    // When materials have enough surface area, relative to their weight, they don't fall in a straight line. This is because the air they are falling in can steer them off course by small amounts. This is a simulation of that effect. Every so often, if the material is traveling fast enough, it will experience a slight offset in position (calculated in pixels).
    fn calculate_pos_offset_from_drag(&mut self) -> Vec2<f64> {
        // how much drag affects an item is dependent on how much gravity it is experiencing
        let drag_grav_ratio = self.material.properties().drag / GRAVITY.pythagorean_theorem();

        // max amount of pixels to offset the material by
        let max_offset_value = 2. * drag_grav_ratio;

        // actual amount to offset material by, random between 0 and `max_offset_value`
        let offset_value = rng::rand_f64((0.)..max_offset_value);

        // materials with less drag need to be traveling faster to have this effect
        let material_is_light_enough =
            self.velocity.x >= (1. - drag_grav_ratio) && self.velocity.y >= (1. - drag_grav_ratio);

        // the time required between pixels shifts
        let offset_delay = Duration::from_secs_f32(0.5);

        if self.last_offset.elapsed() > offset_delay && material_is_light_enough {
            self.last_offset = Instant::now();
            Vec2::new_from_direction(rng::rand_f64((0.)..360.), offset_value)
        } else {
            Vec2::new(0., 0.)
        }
    }
}
