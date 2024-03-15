use std::time::{Duration, Instant};

use crate::{game::rng, rendering::glsl_types::Resolution};

use super::{
    geometry::Vec2,
    id_generator::{Id, IdGenerator},
    material::Material,
    GRAVITY, FRICTION, canvas::Canvas,
};

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
    ) -> Option<(DotCollisionMod, DotCollisionMod)> {
        let next_pos = self.next_position.unwrap();
        if let Some(ref canvas_dot) = canvas.get(next_pos.to_rounded_usize()).unwrap() {
            if self.id != canvas_dot.id {
                Some(self.handle_dot_collision(canvas_dot))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn handle_dot_collision(&self, target_dot: &Dot) -> (DotCollisionMod, DotCollisionMod) {
        let diff = self.velocity - target_dot.velocity;
        (
            DotCollisionMod {
                id: self.id,
                next_velocity: (self.velocity - diff) * (1. - FRICTION),
                next_position: Some(self.position),
            },
            DotCollisionMod {
                id: target_dot.id,
                next_velocity: (target_dot.velocity - diff.to_negative()) * (1. - FRICTION),
                next_position: None
            }
        )
    }

    pub fn set_next_position(&mut self) {
        self.position = self.next_position.take().expect(
            "Next position not set! Don't forget to call `Dot::handle_dot_collision` method",
        );
    }

    pub fn find_next_position(&mut self, resolution: &Resolution, delta_time: &Duration) {
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
        self.next_position = Some(new_position);
    }

    pub fn set_velocity(&mut self, resolution: &Resolution, delta_time: &Duration) {
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

    /// When materials have enough surface area, relative to their weight, they don't fall in a straight line. This is because the air they are falling in can steer them off course by small amounts. This is a simulation of that effect. Every so often, if the material is traveling fast enough, it will experience a slight offset in position (calculated in pixels).
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
