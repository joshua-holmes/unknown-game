use std::time::Duration;

use crate::rendering::glsl_types::Resolution;

use self::geometry::Vec2;

pub mod dot;
pub mod geometry;
pub mod material;
pub mod rng;
pub mod enums;
mod global_game_object;

pub use global_game_object::Game;

// canvas resolution is the size of the game world in pixels
const INITIAL_CANVAS_RESOLUTION: Resolution = Resolution {
    height: 500,
    width: 500,
};

// gravity of every material in the game in pixels per second ^2
const GRAVITY: Vec2<f64> = Vec2 { x: 0., y: 40. };

// while holding mouse button down, delay between pixels that get spawned
const DELAY_BETWEEN_DOTS: Duration = Duration::from_millis(50);
