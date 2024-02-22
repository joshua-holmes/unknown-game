use winit::dpi::PhysicalSize;

pub mod state;
pub mod canvas;
pub mod geometry;
pub mod dot;

// canvas resolution is the size of the game world in pixels
const INITIAL_CANVAS_RESOLUTION: PhysicalSize<u32> = PhysicalSize::new(50, 30);

// gravity of every material in the game in pixels per second ^2
const GRAVITY: f64 = 9.8;

pub fn init() -> state::GameState {
    let canvas = canvas::Canvas::new(INITIAL_CANVAS_RESOLUTION);
    state::GameState::new(canvas)
}
