use winit::dpi::PhysicalSize;

use self::geometry::Vec2;

pub mod canvas;
pub mod dot;
pub mod geometry;
pub mod material;
pub mod state;

// canvas resolution is the size of the game world in pixels
const INITIAL_CANVAS_RESOLUTION: PhysicalSize<u32> = PhysicalSize::new(500, 500);

// gravity of every material in the game in pixels per second ^2
const GRAVITY: Vec2<f64> = Vec2 { x: 0., y: 40. };
