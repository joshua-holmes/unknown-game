use std::time::{Duration, Instant};

use crate::{game::rng, rendering::glsl_types::Resolution};

use super::{
    geometry::Vec2,
    id_generator::{Id, IdGenerator},
    material::Material,
    GRAVITY,
};

/// Special version of `Dot` that contains less data
#[derive(Debug, Clone, Copy)]
pub struct CanvasDot {
    pub id: Id,
    pub material: Material,
    pub velocity: Vec2<f64>,
}
impl From<&Dot> for CanvasDot {
    fn from(value: &Dot) -> Self {
        Self {
            id: value.id,
            material: value.material,
            velocity: value.velocity,
        }
    }
}
impl From<&mut Dot> for CanvasDot {
    fn from(value: &mut Dot) -> Self {
        Self {
            id: value.id,
            material: value.material,
            velocity: value.velocity,
        }
    }
}

pub struct CollisionReport {
    pub this: DotModification,
    pub other: Option<DotModification>,
}

#[derive(Debug)]
pub struct DotModification {
    pub id: Id,
    pub delta_velocity: Option<Vec2<f64>>,
    pub delta_position: Option<Vec2<f64>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Dot {
    pub id: Id,
    pub material: Material,
    pub velocity: Vec2<f64>,
    pub position: Vec2<f64>,
    last_offset: Instant,
}
impl Dot {
    pub fn new(
        dot_id_generator: &mut IdGenerator,
        material: Material,
        position: Vec2<f64>,
        velocity: Vec2<f64>,
    ) -> Self {
        Self {
            id: dot_id_generator.new_id().expect("Ran out of ids"),
            material,
            velocity,
            position,
            last_offset: Instant::now(),
        }
    }

    pub fn find_next_position(&mut self, resolution: Resolution, delta_time: Duration, offset: Vec2<f64>) -> Vec2<f64> {
        // TODO: Add offset back in when physics issues are worked out
        self.velocity * delta_time.as_secs_f64() + self.position
    }

    pub fn find_next_velocity(&self, delta_time: Duration) -> Vec2<f64> {
        let real_drag = self.velocity * self.material.properties().drag;
        let accel = GRAVITY - real_drag;
        let new_velocity = self.velocity + (accel * delta_time.as_secs_f64());

        new_velocity
    }

    /// When materials have enough surface area, relative to their weight, they don't fall in a straight line. This is because the air they are falling in can steer them off course by small amounts. This is a simulation of that effect. Every so often, if the material is traveling fast enough, it will experience a slight offset in position (calculated in pixels).
    pub fn find_pos_offset_from_drag(&mut self) -> Vec2<f64> {
        let drag = self.material.properties().drag;

        // max amount of pixels to offset the material by
        let max_offset_value = drag;

        // actual amount to offset material by, random between 0 and `max_offset_value`
        let offset_value = rng::rand_f64((0.)..max_offset_value);

        // materials with less drag need to be traveling faster to have this effect
        let terminal_velocity = GRAVITY.pythagorean_theorem() / drag;
        let material_is_light_enough = self.velocity.pythagorean_theorem() >= (terminal_velocity * 0.25);

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
