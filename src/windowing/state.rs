use std::sync::Arc;

use winit::{window::Window, dpi::PhysicalPosition};

pub struct WindowState {
    pub window: Arc<Window>,
    pub cursor_position: PhysicalPosition<f64>,
}
