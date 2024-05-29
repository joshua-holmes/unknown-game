use std::time::Duration;

use crate::rendering::glsl_types::Resolution;

use self::vec2::Vec2;

pub mod dot;
pub mod geometry;
pub mod material;
pub mod rng;
pub mod id_generator;
pub mod canvas;
pub mod vec2;
mod global_game_object;

pub use global_game_object::Game;

// canvas resolution is the size of the game world in pixels
const INITIAL_CANVAS_RESOLUTION: Resolution = Resolution {
    height: 500,
    width: 500,
};

// gravity of every material in the game in pixels per second ^2
const GRAVITY: Vec2<f64> = Vec2 { x: 0., y: 100. };

// while holding mouse button down, delay between pixels that get spawned
const DELAY_BETWEEN_DOTS: Duration = Duration::from_millis(50);

// percentage of energy lost due to friction when 2 dots collide
// acceptable range is 0.0 to 1.0, higher is more friction
const FRICTION: f64 = 0.0;

// size of cursor that spawns dots; this is the radius of the cursor
const CURSOR_SIZE: f64 = 5.;

