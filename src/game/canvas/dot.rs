use std::time::{Duration, Instant};

use crate::game::{
    material::Material,
    math::rng,
    Vec2, GRAVITY,
};
use super::{DotIdGen, DotId};

/// Special version of `Dot` that contains less data
#[derive(Debug, Clone, Copy)]
pub struct CanvasDot {
    pub id: DotId,
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

#[derive(Debug, Clone, Copy)]
pub struct CollisionReport {
    pub this: DotModification,
    pub other: Option<DotModification>,
}

#[derive(Debug, Clone, Copy)]
pub struct DotModification {
    pub id: DotId,
    pub delta_velocity: Option<Vec2<f64>>,
    pub delta_position: Option<Vec2<f64>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Dot {
    pub id: DotId,
    pub material: Material,
    pub velocity: Vec2<f64>,
    pub position: Vec2<f64>,
}
impl Dot {
    pub fn find_next_position(&self, delta_time: Duration) -> Vec2<f64> {
        self.velocity * delta_time.as_secs_f64() + self.position
    }

    pub fn find_next_velocity(&self, delta_time: Duration) -> Vec2<f64> {
        let real_drag = self.velocity * 2. * self.material.properties().drag;
        let accel = GRAVITY - real_drag;
        let new_velocity = self.velocity + (accel * delta_time.as_secs_f64());

        new_velocity
    }
}
