use winit::dpi::PhysicalSize;

pub mod state;
pub mod canvas;

// canvas resolution is the size of the game world in pixels
const INITIAL_CANVAS_RESOLUTION: PhysicalSize<u32> = PhysicalSize::new(10, 4);

pub fn init() -> state::GameState {
    let canvas = canvas::Canvas::new(INITIAL_CANVAS_RESOLUTION);
    state::GameState {
        canvas
    }
}
