use std::time::{Duration, Instant};

use crate::{game::rng, rendering::glsl_types::Resolution};

use super::{
    geometry::Vec2,
    id_generator::{Id, IdGenerator},
    material::Material,
    GRAVITY, FRICTION, canvas::Canvas,
};

pub struct DotCollisionMods {
    pub this: DotCollisionMod,
    pub other: DotCollisionMod,
}

pub struct DotCollisionMod {
    pub id: Id,
    pub next_velocity: Vec2<f64>,
    pub next_position: Option<Vec2<f64>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Dot {
    pub id: Id,
    pub material: Material,
    pub velocity: Vec2<f64>,
    pub position: Vec2<f64>,
    pub next_position: Option<Vec2<f64>>,
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
            next_position: None,
            last_offset: Instant::now(),
        }
    }

    pub fn check_for_dot_collision(
        &self,
        canvas: &mut Canvas,
    ) -> Option<DotCollisionMods> {
        let next_pos = self.next_position.unwrap();
        let ray = canvas.cast_ray(self.position, next_pos);
        let mut prev_coord = self.position.to_rounded_usize();
        for point in ray {
            if let Some(target_dot) = point.dot {
                if self.id != target_dot.id {
                    let diff = self.velocity - target_dot.velocity;
                    return Some(DotCollisionMods {
                        this: DotCollisionMod {
                            id: self.id,
                            next_velocity: (self.velocity - diff) * (1. - FRICTION),
                            next_position: Some(prev_coord.into_f64()),
                        },
                        other: DotCollisionMod {
                            id: target_dot.id,
                            next_velocity: (target_dot.velocity - diff.to_negative()) * (1. - FRICTION),
                            next_position: None
                        }
                    });
                }
            } else {
                prev_coord = point.coord;
            }
        }
        None
    }

    pub fn find_next_position(&mut self, resolution: Resolution, delta_time: Duration, offset: Vec2<f64>) -> Vec2<f64> {
        let unclamped_position =
            self.velocity * delta_time.as_secs_f64() + offset + self.position;
        let new_position = unclamped_position.clamp_to_resolution(resolution);
        new_position
    }

    pub fn find_next_velocity(&self, resolution: Resolution, delta_time: Duration) -> Vec2<f64> {
        let real_drag = self.velocity * self.material.properties().drag;
        let accel = GRAVITY - real_drag;
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
