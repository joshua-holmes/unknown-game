use std::sync::Arc;

use winit::{window::Window, dpi::PhysicalPosition, event_loop::EventLoop};

pub struct WindowState {
    pub window: Arc<Window>,
    pub cursor_position: PhysicalPosition<f64>,
}
impl WindowState {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = Arc::new(Window::new(event_loop).unwrap());
        Self {
            window,
            cursor_position: PhysicalPosition::new(0., 0.),
        }
    }
}
