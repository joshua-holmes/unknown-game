use winit::dpi::PhysicalSize;

use self::{dot::Dot, material::Material, geometry::Vec2};

pub mod state;
pub mod canvas;
pub mod geometry;
pub mod dot;
pub mod material;

// canvas resolution is the size of the game world in pixels
const INITIAL_CANVAS_RESOLUTION: PhysicalSize<u32> = PhysicalSize::new(500, 500);

// gravity of every material in the game in pixels per second ^2
const GRAVITY: Vec2<f64> = Vec2::new(0., 9.8);

pub fn init() -> state::GameState {
    let mut canvas = canvas::Canvas::new(INITIAL_CANVAS_RESOLUTION);
    
    // Set some dots for testing
    canvas.dots.push(Dot {
        material: Material::Sand,
        position: Vec2::new(0., 0.),
        velocity: Vec2::new(100., 0.),
    });
    //

    state::GameState::new(canvas)
}
